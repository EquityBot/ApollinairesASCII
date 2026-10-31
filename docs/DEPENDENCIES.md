# Dependency scan and reachability

Checked on 1 November 2026 against the committed Cargo.lock. Scan alerts describe package versions; reachability conclusions below concern ApollinairesASCII's current calls and do not declare those packages free of vulnerabilities.

## Fixed reachable decoder alert

Aikido reported `AIKIDO-2026-10928` in `weezl` 0.1.12. GIF input reaches the LZW decoder, so file limits alone do not remove the exposure. The [official GIF upstream change](https://github.com/image-rs/image-gif/commit/e60e4dd7708cc50f9371185cefa04273f410f116) updates it to 0.2.1. Cargo.toml pins that exact upstream commit until a stable GIF release includes it. Cargo.lock now resolves `weezl` 0.2.1 and a subsequent Aikido scan no longer reports this alert. [Upstream changes](https://github.com/image-rs/weezl/blob/v0.2.0/Changes.md#version-020) describe malformed-stream and progress fixes.

## Remaining package alerts

| Locked package | Aikido alerts | Current application reachability |
| --- | --- | --- |
| `zip` 0.6.6, through `figlet-rs` 1.0.0 | `CVE-2026-115905`, `CVE-2026-115906`, `AIKIDO-2026-476773`, `AIKIDO-2026-306911` | Archive handling is not called. Only validated plain FLF content and embedded fonts reach `FIGlet::from_content` / built-in methods. |
| `moxcms` 0.8.1, through `image` 0.25.10 | `AIKIDO-2026-175889` | Its affected ICC byte parser is not called. Conversion decodes pixels without interpreting embedded ICC profiles. |

The ZIP dependency's file/byte loading APIs invoke archive support; ApollinairesASCII does not expose or call those APIs. Its CLI reads a font as bounded UTF-8 text and passes it through the same plain-font validator as the browser. Compressed font archives are refused. See the [ZIP publisher advisory](https://github.com/zip-rs/zip2/security/advisories/GHSA-m665-v7pm-7grj).

`DynamicImage::from_decoder` in the locked image crate decodes pixels and assumes sRGB. It does not pass embedded ICC bytes to `moxcms::ColorProfile::new_from_slice` or `new_from_slice_with_options`. Image's profile construction uses built-in colour metadata. This supports non-reachability of the specific ICC LUT parsing alert; it is not a conclusion about every colour-processing defect. See the [moxcms 0.9.0 fixes](https://github.com/awxkee/moxcms/releases/tag/0.9.0).

At this check, the latest stable `image` and `figlet-rs` releases still require the older dependency ranges. The 5 alerts remain visible and are not ignored or snoozed in the scanner. Reassess these conclusions when adding archive APIs, ICC colour management or changing the dependency graph. Upgrade through upstream releases when compatible fixes become available.

## Source scan

Aikido scanned the 11 Rust source files, browser HTML/CSS and 2 GitHub configuration files: 15 files, zero issues and zero skipped paths. This automated result complements the contract tests and independent code reviews; it is not an assurance that every possible defect was found.
