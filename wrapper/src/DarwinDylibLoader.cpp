#include "ctp-rs/wrapper/include/DarwinDylibLoader.h"

#include <cerrno>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <dlfcn.h>
#include <fcntl.h>
#include <sys/types.h>
#include <unistd.h>

namespace ctp_rs::darwin {

void* extract_and_dlopen(const char* name,
                         const unsigned char* blob_start,
                         const unsigned char* blob_end) {
    const std::size_t size = static_cast<std::size_t>(blob_end - blob_start);
    if (size == 0) {
        std::fprintf(stderr,
            "ctp-rs: embedded %s dylib has zero size; build.rs is "
            "misconfigured\n",
            name);
        return nullptr;
    }

    // Per-process temp file. We deliberately keep the file around for the
    // lifetime of the process — dlopen() retains a reference to the inode,
    // unlinking before dlopen() can race the loader on some macOS versions,
    // and unlinking after means a lingering file across crashes. Extracting
    // once per pid and letting the OS's tmp cleaner reap it is the cleanest
    // model.
    const char* tmpdir = std::getenv("TMPDIR");
    if (!tmpdir || !*tmpdir) tmpdir = "/tmp";
    char path[1024];
    std::snprintf(path, sizeof path, "%s/ctp-rs.%d.%s.dylib",
        tmpdir, static_cast<int>(getpid()), name);

    int fd = ::open(path, O_WRONLY | O_CREAT | O_TRUNC, 0600);
    if (fd < 0) {
        std::fprintf(stderr, "ctp-rs: open(%s): %s\n", path,
            std::strerror(errno));
        return nullptr;
    }
    std::size_t written = 0;
    while (written < size) {
        ssize_t n = ::write(fd, blob_start + written, size - written);
        if (n < 0) {
            if (errno == EINTR) continue;
            std::fprintf(stderr, "ctp-rs: write(%s): %s\n", path,
                std::strerror(errno));
            ::close(fd);
            ::unlink(path);
            return nullptr;
        }
        written += static_cast<std::size_t>(n);
    }
    if (::close(fd) != 0) {
        std::fprintf(stderr, "ctp-rs: close(%s): %s\n", path,
            std::strerror(errno));
        ::unlink(path);
        return nullptr;
    }

    // RTLD_LOCAL keeps each dylib's symbols out of the global namespace so
    // md and trader copies of shared CTP utility code can't collide.
    void* h = ::dlopen(path, RTLD_NOW | RTLD_LOCAL);
    if (!h) {
        const char* err = ::dlerror();
        std::fprintf(stderr, "ctp-rs: dlopen(%s): %s\n", path,
            err ? err : "<no dlerror>");
        ::unlink(path);
        return nullptr;
    }
    return h;
}

} // namespace ctp_rs::darwin
