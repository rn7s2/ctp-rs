#!/usr/bin/env python3
"""Regenerate wrapper/include/DarwinSdkShim.h from the darwin SDK headers.

The shim mirrors the darwin trader-side TraderApi/Spi vtable layout so that
the macOS-dlopen build inherits/dispatches against a class whose vtable
matches what the dlopened framework dylib was built against. Re-run this
whenever build.rs's LIB_ASSET_URL is bumped to a different darwin SDK
version.

Usage:
    python3 wrapper/scripts/regen_darwin_shim.py <darwin-sdk-headers-dir>

Where <darwin-sdk-headers-dir> contains ThostFtdcTraderApi.h. Typically
that's $OUT_DIR/lib/darwin/thosttraderapi_se.framework/Versions/A/Headers/
of a recent build.
"""
import re
import sys
from pathlib import Path

SHIM_PATH = Path(__file__).resolve().parent.parent / "include" / "DarwinSdkShim.h"


def extract_class(text: str, marker: str) -> str:
    out = []
    flag = False
    for line in text.splitlines():
        if marker in line and line.lstrip().startswith("class"):
            flag = True
        if flag:
            out.append(line)
            if line.startswith("};"):
                break
    return "\n".join(out)


def virt_lines(class_text: str) -> list[str]:
    out = []
    for line in class_text.splitlines():
        s = line.strip()
        if not s.startswith("virtual"):
            continue
        s2 = re.sub(r"\{[^}]*\}\s*;?\s*$", "", s)
        s2 = re.sub(r"=\s*0\s*;?\s*$", "", s2)
        s2 = s2.rstrip().rstrip(";").rstrip()
        out.append("    " + s2 + " = 0;")
    return out


def main(headers_dir: str) -> None:
    hdr = Path(headers_dir) / "ThostFtdcTraderApi.h"
    text = hdr.read_text(encoding="utf-8")
    api_class = extract_class(text, "CThostFtdcTraderApi")
    spi_class = extract_class(text, "CThostFtdcTraderSpi")
    api_v = virt_lines(api_class)
    spi_v = virt_lines(spi_class)
    if not api_v or not spi_v:
        sys.exit(f"Could not extract virtual methods from {hdr}")

    out = [
        "// AUTO-GENERATED from the darwin SDK headers — see wrapper/scripts/regen_darwin_shim.py.",
        "// Mirrors the darwin trader-side TraderApi/Spi vtable layout so that on the",
        "// macOS-dlopen build, the wrapper inherits/dispatches against a class whose",
        "// vtable matches what the dlopened framework dylib was built against.",
        "//",
        "// Linux 6.7.11 added a handful of API + SPI methods that the older darwin",
        "// SDK lacks; using the linux base classes directly leaves the vtables",
        "// misaligned for every slot past the first divergence, causing the dylib to",
        "// call the wrong override on us and us to call the wrong slot on the dylib.",
        "//",
        "// Compiled only on the macOS-dlopen build (where CTP_RS_DARWIN_TRADER_DLOPEN",
        "// is defined). Field types come from the linux SDK headers — the small set",
        "// of structs whose layout differs (RspUserLogin, TradingAccount,",
        "// SyncingTradingAccount, SyncDeltaTradingAccount) is passed by pointer here",
        "// and widened/narrowed at the conversion boundary in CTraderSpi.cpp.",
        "",
        "#pragma once",
        "",
        '#include "ThostFtdcUserApiStruct.h"',
        '#include "ThostFtdcUserApiDataType.h"',
        "",
        "class CThostFtdcTraderSpiDarwinShim;",
        "",
        "class CThostFtdcTraderApiDarwinShim {",
        "public:",
    ]
    for v in api_v:
        if "RegisterSpi" in v and "CThostFtdcTraderSpi" in v:
            v = v.replace("CThostFtdcTraderSpi", "CThostFtdcTraderSpiDarwinShim")
        out.append(v)
    out += [
        "};",
        "",
        "class CThostFtdcTraderSpiDarwinShim {",
        "public:",
    ]
    out += spi_v
    out += ["};", ""]

    SHIM_PATH.write_text("\n".join(out))
    print(f"wrote {SHIM_PATH} ({len(api_v)} API + {len(spi_v)} SPI virtuals)")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
