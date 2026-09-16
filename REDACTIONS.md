# Publication provenance

## Evaluator status update, September 16, 2026

[Current qualification status](QUALIFICATION-STATUS.md) adds a dated index of
existing public replay packages and the latest internal release progress. It
records candidate source `9467eb1d2cb9aea2625f3e6b6f8d189d447c4172` separately
from the completed earlier baseline. The 06:56 UTC protected-node snapshot is
preserved with a pointer to the update. README and verification-guide links
make the current record discoverable.

This is an editorial update. All raw artifacts, complete manifests and verifier
sources are unchanged from public commit
`2f6373696bffc48fd1fced10d5e78be3942824d8`. Internal execution status is labeled
as such; the update publishes no private engine source, keys, runtime stores or
new raw test package.

## Live HKDF migration package, September 16, 2026

The [live HKDF package](migration/live-hkdf-2026-09-15/) retains both the rejected
R5 run and accepted R6 run. All nine independent-auditor inputs for each run,
their original audit reports, exit files, chain results and pre-copy oracle
receipts are byte-identical copies. No authority counters, request identities,
timestamps, response bytes or oracle fields were redacted or replaced. The
[original/public provenance table](migration/live-hkdf-2026-09-15/ORIGINAL-PROVENANCE.json)
records the file identities and hashes.

The four auditor Rust/Cargo files and its README are unchanged. The new offline
runner compiles that auditor and requires its original 31 controls, R5 rejection
and R6 acceptance. Public dependency sources retain their original licenses and
Cargo checksums. [CUSTODY-PINS.json](migration/live-hkdf-2026-09-15/CUSTODY-PINS.json)
records engine identities and original closed-corpus digests. The original
auditor build receipt contains local absolute paths and is retained by digest;
its source/executable pins are reproduced without those paths.

The R6 host-context JSON is a clearly identified derivative of recorded CPU,
memory, load and kernel receipts, with their original hashes. It contains
relevant machine characteristics and omits private host identifiers. Private
engine source, executable, protected stores, authority databases and root
material are excluded from the package.

The new complete package `SHA256SUMS` includes Rust, Cargo, typed fixtures and
vendored dependencies. The updated root raw-artifact manifest also pins that
package manifest; the two verifiers retain separate scopes. Existing raw
cryptographic and two-host recovery artifacts are unchanged from public main
`213f394cd8d5fcc19156201e216cd9c422a7d2e8`. Root README, claims and verification
instructions now link the completed live result and its negative predecessor.

## Required recovery publication, September 15, 2026

The new [two-host recovery package](mesh/required-two-host-2026-09-15/) publishes
a synthetic 512-record campaign. Its [original-provenance table](mesh/required-two-host-2026-09-15/ORIGINAL-PROVENANCE.json)
records every original/public hash and transformation. Expected record data,
clock reports, the host A wrapper and the wire report are byte-identical
originals. The two final/open manifests omit a private binary evidence encoding
and an internal API identifier. Local data/authority paths become hashes of
their original UTF-8 values; the B executable path and private test module name
become explicit labels. The remaining typed values and reported outcomes are
preserved. Newly authored UTC timestamps use `Z`; offset normalization preserves
the recorded instant.

No pre-existing raw artifact changes with this addition. The full inventory
now includes `.jsonl` so all four new JSON-line artifacts are covered, taking
the raw-file count from 249 to 267. The new Rust verifier is an artifact reader,
not the 8DB implementation. The package excludes seeds, credentials, private
stores, authority-floor bytes, raw packet captures, private local paths and
engine source. Earlier negative development and coordination outcomes remain
in [the result history](mesh/required-two-host-2026-09-15/negative-history.json).

## Earlier publication history

This record separates the original public evidence snapshots from later edits
to the accompanying documents. The original publication removed identifiers
from selected files: instance and runner names, private addresses, local paths,
one personal email address and internal tracking references. The table below
retains the original before-and-after hashes.

The original run manifests refer to the internal evidence files. For a redacted
public copy, use the original publication table to reconcile its hash. For a
document revised on September 14, use the editorial table that follows it.

## Original publication snapshots

This table is preserved from repository commit
`6b4675999dc17b9afb6f3881e1eb572e4d9396d8`. Its "Published SHA-256" values
identify that publication snapshot, including documents subsequently revised.
They remain part of the history and are not replaced with editorial hashes.

| File | Original SHA-256 | Published SHA-256 |
|---|---|---|
| `acvp/EVALUATOR-KIT.md` | `186301b50886d15d7f568c3e6b48975d3cebbe2c04098b52e00cfc8972704f49` | `5ddcb323e655e93d3ddb0b51de3fc267af021eec25ab6b4f066dcdd267c50c9a` |
| `acvp/HARNESS-README.md` | `0f703a8fcba4328dbb3e086bc6ed2da1a6ee4461ad4eb07b3b27adef1eca1e64` | `e5b94665275e49109d91128ba4529f050ab46303f24aaf006dd85a7e0b43ee13` |
| `acvp/NIST-ACVP-Disposition-2026-06-16.json` | `3eb96b828117d4765417b08e10b2f048607fd1435c5f42eb8a0d56f7da063ffd` | `ee001bae16e22a5f0783b62476c2955051f790c26272015e252f568aa9d72406` |
| `bench/cnsa_m1_release_aarch64-linux_2026-09-12.txt` | `3ed7db0213130cb19589f9d2f731db5bbd69dce8fa9b28f578058d5895758d90` | `c460088ecc77b3e3a33ceae0bc1395a5018eb276e22d7c805d681d9bef1ca406` |
| `interop/README.md` | `dc01f8a652cd00e80a994de142ed380fc2b42a65e16ce492b73a5f9c63850614` | `6a16031ab81fb66a656160bd66da454dba98a6a3a5cf420612b90cc00117d3d9` |
| `interop/aarch64-linux_2026-09-12/host.txt` | `eaa96ccfbe5f233285cf8b4beed06a9738ad9a4086b80d2ea45d31147ee4a75c` | `6cb2815c1ec7131270b82782fcd9631f9880f8ca3ca896d262fa753801878cc9` |
| `interop/x86_64-windows_2026-09-11/host.txt` | `5c6c439b7ee5aa245e255faffe779ac655f37ab77d14236d914f58ab45d43cb3` | `f711ba4809cf9c4b7c71fc5e34c874d64c4757bf5fae2f25b41829e352fe3293` |
| `timing/README.md` | `561e981142fc02109dc461b74e5a808c06f8efbbc9f68deb2e219a8b317de58d` | `9e7c4f20c23876e2edefd00c83b2d03dd3c9ab44c8dd65a4392ae3c50d5aa863` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/NOTE.md` | `af576335e57658a953d63d0b5ee632f93b28455ede6d8ca46c74b4015425024e` | `966b88fb16ad57f6fc5e0a46e4ba3d5148804e366d930311b91f1afbb00a0d63` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/build.txt` | `12ca6da784d404faffd7a29887f468b5a09ed76d158b9af8fe50a51a282d695d` | `97e5998f03a3cdb59ef5a4b4a8eea37219fcd08459cdf72778c5399371cae46f` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/host.txt` | `752be97c0b31b2ad62fac7bdd2c056cf4c3968252d8a76d6b2652bbb35bd33e5` | `8e808ec4801eb236bbfc19822d4e3f8d8bbad777c6984b2b3817dd2fdac33ed1` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/report.txt` | `f708203ccee62fb8b9762914a1e80fe728bb02dcc1892956c458c6e1a77cc721` | `31e99714460edf4acc824fc8df3fbd821910eb5f5cefad20d94af59121614237` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/suite.txt` | `9588e66739fe23896828f2354e402ffc996579e11b1b1408d9d370bd9b3eabde` | `63fa4a7e9930166702aa52c207960245cd592926ff54db3bda181831426a0ee1` |
| `timing/aarch64-linux_2026-09-12_run2-suite-v3/build.txt` | `fdde810b89b680807974c1e8c3beecf6c0c06d918347475311c4572796f42df9` | `59db3f3d5f33165290d785e35dc12f851f19d51f9416fed0addbe5c9ca6ce982` |
| `timing/aarch64-linux_2026-09-12_run2-suite-v3/host.txt` | `273927ad8b2b274b1648021513b5ef7cc6561a279d33856d4b1ac6bcb7d2be6e` | `0e2d07a5e5de60c489f349443d2fffc85a023a4c7a87e9cf60d45306a56dcf3c` |
| `timing/aarch64-linux_2026-09-12_run2-suite-v3/report.txt` | `3c4e9b17a6cfecd73af43270b795fe64ded7add993fc5434a616350eac875a86` | `07ee8be4aa8af45a6dd064da2f296dc32adc537b07ff367646e85d4de34a72c4` |
| `timing/aarch64-linux_2026-09-12_run2-suite-v3/suite.txt` | `e6552ad57f68f9f58cd9925923503deb6d2623ebbeed9d6e92b0579de1c1379e` | `1dd5f2cb7510e930601713db805fbbe75dc535d0075d6cc91760499c8cbbd409` |
| `timing/aarch64-linux_2026-09-12_run3-harness-v2.1/build.txt` | `6e899f8100902a325e136453958dcc91bc8e7bf9c0067707a4492853cecd2f52` | `ebdbdd37cf571b9120ff4df874a6d8a8b55ac773e1c3a75faf6aa96bb8eab4b9` |
| `timing/aarch64-linux_2026-09-12_run3-harness-v2.1/host.txt` | `5a9e3d23c84fa7a4526866742a672b56c1366ae72e84309d12184649374ad639` | `5f50160767353940d8e7f15cb41455e32810ba3c5d17e25684d93b6d20a71702` |
| `timing/aarch64-linux_2026-09-12_run3-harness-v2.1/report.txt` | `561f4953f422770a631788e3ab3b78ccee8e686da1908f20c7934a465d3f66fb` | `8a1c17adc4db90545988da9d52733fe2f3df58c6a2db4b81addc4da02d65a69f` |
| `timing/aarch64-linux_2026-09-12_run3-harness-v2.1/suite.txt` | `72136b45fb89a50e3b788889f337385cb6cae697012341af0e361acd4e9a401f` | `4f3e3b917f09860456d178823f5422c34a870bb71311acf2e2432b3fdfda4df7` |
| `timing/aarch64-linux_2026-09-14_run4-fips/build.txt` | `ffbb66dcb5f40d4fd9708bddf31c207a6c1c6360ce7bcac38f784765907ec057` | `bbec11a5319d02b5ac69cf72930dcceaa3f8cebb73ead71cdc33cebc1d27a8f0` |
| `timing/aarch64-linux_2026-09-14_run4-fips/host.txt` | `dfbb8205d1f490b50cb0ee65e5df5d65b8312dfc35b2ee7eb3ab094cbb2f49af` | `656a7eca223667a6cd0113b3dd05b35531552dc72f703c97c7e3e3ce2a4290c6` |
| `timing/aarch64-linux_2026-09-14_run4-fips/report.txt` | `c91b06e1cb9c98bc4f0e1d7baab7e3fddc54b48b6b6bab4d16b24c1d532b3186` | `4c9f8ab4a51abada24aec5d062c124d2bf1bc3ba93f9993b66a3668b4c3973b0` |
| `timing/aarch64-linux_2026-09-14_run4-fips/suite.txt` | `33f31594aaf1012cf1893b23c207d5fcf39574dd917fb7b9f532e677e88fc654` | `3cf8a16487ecaac0a9e6bf826e6cc3db3bfd230942dbe3aded331b04d1e6bf4b` |
| `timing/x86_64-linux_2026-09-12_run1-harness-v2.1/build.txt` | `c39ce813571fdce8791bf4cb885d8fa7599278e2750e3695f578e325fb000e6d` | `0b2604c1a96e2566196a4da719f924cb97f023cd42966a3e3bb40f201876f572` |
| `timing/x86_64-linux_2026-09-12_run1-harness-v2.1/host.txt` | `823f7e15adeddd7bbde7d64d2688c2828b1ae3dfd6ad4d86f27327dc8f4aeb39` | `41dc5260fa185742d3647650c0eed736affde9fa70ad9932c7554607db8af9c2` |
| `timing/x86_64-linux_2026-09-12_run1-harness-v2.1/report.txt` | `4e0ca43eb8d9b2d3a1ed76891110a2443f0c53a7717a28992762d11d6caac663` | `5e55147649428bb6242f82404670f4119bf150d2ad34c8e82529847fe22800a0` |
| `timing/x86_64-linux_2026-09-12_run1-harness-v2.1/suite.txt` | `b92a079556af4663fec680660de8947f7be7712d2e894286d30aa986045b5801` | `dbc472b49e40cbc7c7e75ca8432f72818932b5c2da2691dc48edc1ab92cc02e1` |
| `timing/x86_64-linux_2026-09-14_run2-fips/build.txt` | `8f4d608593acc3d01f36a75bfe720880030adbf72b6aebfb62ad767de6359e5c` | `72aee1025b632a3807f020d9822b9c254a177db8dd0ab65ce010ed9bf54f8ce4` |
| `timing/x86_64-linux_2026-09-14_run2-fips/host.txt` | `2e7ebfbfce9b88f1d0aef0ad236888760245679b1d0be410540ae484d2b3ac5a` | `ca3dec7730a2c7b82766d18b01b8c864f852d661076bb9427aee7bda42f3ed56` |
| `timing/x86_64-linux_2026-09-14_run2-fips/report.txt` | `bb57b92fdbac8b1dcb8ac96a51b80532c6ae88ba11322ddea5d429f8a8538763` | `585121d92c1708b5b608e6bb2d4ed44beaeaf98900f3c6d5359fe423a3cc62aa` |
| `timing/x86_64-linux_2026-09-14_run2-fips/suite.txt` | `16e81240f3382b54b92060cd3727377c0ed6decabe3ceea61a502dcb68c33208` | `164e3a3223fa4686b4b22d504535acc639eaf944a68fb0f771bc951b73a54710` |
| `timing/x86_64-linux_2026-09-14_run3-fips-200k-invalid/build.txt` | `fc16e936d11e27d69828c18bc8f39780f130174c3dcee6a810f29f16af9544d6` | `0a3fd3d353858a268001770277fd75935b1794cd89943d3083e66da0a8c75ba7` |
| `timing/x86_64-linux_2026-09-14_run3-fips-200k-invalid/host.txt` | `b1b98fb3bfc1e2d3bba794941efe14d19e2523a5855bb74846bb53893a5ca2a5` | `2f12f6bd8e964fb3707796435e25b2ced60511659113d02a97c07d92fe85d739` |
| `timing/x86_64-linux_2026-09-14_run3-fips-200k-invalid/report.txt` | `ac8e7152ab3fa4e7aa35af5e0adaf3d025e1de2113be6d44db1537891bb85010` | `353a4c272e7d118919619d3e65defd7fe85cbc69dac6d2052271e2ad946848bf` |
| `timing/x86_64-linux_2026-09-14_run3-fips-200k-invalid/suite.txt` | `14a7aa6ba1829619bf15765398ae117b565943c52fab2471b91a60f465ba728a` | `b92092f395d5076bc2bbb6f4fcc882ac0d35366cd36ab0e40162b1795335c5b0` |
| `timing/x86_64-windows_2026-09-12_run1-harness-v1/build.txt` | `c7cfe95cbfe3dc4249846888a90a6f91a13fca9a82671ddfab617b514fb460d5` | `bd7f01d2084c39023a02bb1a26a8d4428290416f80fef9a62e5d1234c21a6adf` |
| `timing/x86_64-windows_2026-09-12_run1-harness-v1/report.txt` | `3d56b83698bcff0d9ce7d0b252250bac6d5445eafd20cfa2fb3defa511ef77a3` | `856eb420fa5198deedbfd7d84ac8d880550136135dcbf7ba5f995044690e589b` |
| `timing/x86_64-windows_2026-09-12_run1-harness-v1/suite.txt` | `15949037184698feb8ae6636a6bab8872ad32ef820dec70cb98993972c7d384a` | `1789e7d52049174e4583363133ab3ebd16be775f21be5ef6504a7d195ba989ab` |
| `timing/x86_64-windows_2026-09-12_run2-harness-v2/build.txt` | `43663f185107a8adbc7cf92ebfd952fd84c4e81a466663e3d03ee06dcbcde1e1` | `bfcdc157dc75cced97d245bf7cf772425a4ef96971ec0f110005a4e0036d3c54` |
| `timing/x86_64-windows_2026-09-12_run2-harness-v2/report.txt` | `d3a656f0cf05e1594210cd366a037c6e821371348bbd0ea2aa9589b927447762` | `2e3e590210c940fda9fd4b6cacf8d499daf879b3c92cb323d76f9aadc6bd6596` |
| `timing/x86_64-windows_2026-09-12_run2-harness-v2/suite.txt` | `c7d77292d82af1ccee74ad5b23db53bbb895edc616b0ae22098ceeff895d486a` | `86446c243b45f2763d4f000a64902049e27fa10b2f869104a249dc968127a802` |
| `timing/x86_64-windows_2026-09-12_run3-suite-v3/build.txt` | `aa14e851c95b28726132f3b6eeaa4bd08fbcdb8c271c95c56baf171542a54857` | `2cd61479caf601c8a2e228239aee52d47eb0ba73f1133acd539c46424810aa34` |
| `timing/x86_64-windows_2026-09-12_run3-suite-v3/report.txt` | `e5fb1c663e97c40bcab76c4c072f71502bfdda7ecc565e1125faa2481c48759a` | `b1799a88d2b39d2a722e76a5298a3b9bb32cb3e8f8773ffbc84802778b727d8d` |
| `timing/x86_64-windows_2026-09-12_run3-suite-v3/suite.txt` | `2f5798c0566c512a46033ebbee17c75087b3a11dc1174e074280da2de12a8f63` | `4f2acf315801109c82e222558d452d1f3ce4b370f4f344fecc603c9283eb7b98` |
| `timing/x86_64-windows_2026-09-12_run4-harness-v2.1/build.txt` | `5dd0d164007bb3fdc48e07c3ce01596f01d5877af120bb40c16072f7dd49e694` | `9327861c89ad8ad601c06509ba19533687fd85a3027df9d0034b25f81a9a5c2c` |
| `timing/x86_64-windows_2026-09-12_run4-harness-v2.1/report.txt` | `1311b4ce6f68f820eac8fbe432e0d848fe31867905ee15b98b4beccbe72501a1` | `1cf6724d5be3b4c7bc354ead0d5ff8849eb3af9110b41287454b14130c837c91` |
| `timing/x86_64-windows_2026-09-12_run4-harness-v2.1/suite.txt` | `5be0914b4ee3b5d63f00e25cdbdd68cd1fc07f1d35f70cc02f085584e8b3dbc3` | `11ab04cf1ac7416659d21828c73ed6a9db76d20f1983aefc2f029be706148da1` |

## Editorial revisions, September 14, 2026

The following revisions improve the presentation of 8DB, clarify review and
reproduction instructions, and correct the interpretation of existing evidence.
They preserve the recorded measurements and identify unresolved timing findings.
All pre-existing non-Markdown files remain byte-identical to the baseline commit, including
the vectors, raw reports, transcripts, binary artifacts and run manifests.

"Before revision" is the SHA-256 of the exact file bytes at commit
`6b4675999dc17b9afb6f3881e1eb572e4d9396d8`. "After revision" is the SHA-256
of the revised file bytes in this edition. This provenance document is excluded
from its own table because including its hash would make the table self-referential.

| File | Before revision SHA-256 | After revision SHA-256 |
|---|---|---|
| `acvp/EVALUATOR-KIT.md` | `5ddcb323e655e93d3ddb0b51de3fc267af021eec25ab6b4f066dcdd267c50c9a` | `a2b65abfd2842998452e6a6b18b98cc52a9d91da31d992d267b9307ef42a9974` |
| `acvp/HARNESS-README.md` | `e5b94665275e49109d91128ba4529f050ab46303f24aaf006dd85a7e0b43ee13` | `5a19f919636a6ceb08bc84c5af47bb4c27c6f1031e9bccad1220c74de00a8cf9` |
| `CLAIMS-AND-SCOPE.md` | `afa64b5e7e6263cab77630072bf247ee76021d393c88258e9397d77fdf8533f2` | `f50e54f615a9ef33cc249ba7f5cb912e7608d2febeab505bf37e96b08ae8abc6` |
| `HOW-TO-VERIFY.md` | `a3efb5d07b5cdb27d2a7fc4fa0128abf405b7fc0ee52667e771fc75bc8859a9f` | `20d28a0982e01e6f25aedade94b6beb202b05b04a29c4abdb0e14584872fd3d1` |
| `interop/README.md` | `6a16031ab81fb66a656160bd66da454dba98a6a3a5cf420612b90cc00117d3d9` | `a76de41196f281ad30d2287a9c7c25527fa815ce707ba3e1843cacdbe817d559` |
| `kat/ml-dsa-87/ml_dsa_87_nist_sig_SOURCE.md` | `24b900340155eb2902602e99f5b51a2bfe426d78f86e5b6e33beb5c2e79c336a` | `e6af0b7652cd6711afc92f8cfc5fc6fa21fd9f21e969220d23be45ae29cd9553` |
| `README.md` | `2a0b3fb9538d3c4dfb808f28ddd8768f4ac0eac0ae2ac05348995fa074829e4f` | `682946f882d047c501e406f4ab4c1ebf5dd71078534de1b6ecadc12d2b632dde` |
| `timing/aarch64-linux_2026-09-12_run1-harness-v1/NOTE.md` | `966b88fb16ad57f6fc5e0a46e4ba3d5148804e366d930311b91f1afbb00a0d63` | `241109042615f62a6aff27467bf899ca88cc36a50c1bdf21aed8492dc28a5cc8` |
| `timing/README.md` | `9e7c4f20c23876e2edefd00c83b2d03dd3c9ab44c8dd65a4392ae3c50d5aa863` | `5f9f39698f63807882c8817a791d47ad7c69904cc8d4ab4bdef642709351873c` |
| `timing/x86_64-windows_2026-09-12_run1-harness-v1/NOTE.md` | `b3bf4ce8053e3b8497c4aaee02b29e5c37150e499b55c3779bc8e802f31dbdf1` | `8168a6fc3da683b47694bba2544c9e0b859f792195e6bc6f30c1fc2c2f5f34fc` |

## Public verification tooling

The following files were added with this edition. The artifact manifest records
hashes of the published raw files, which lets readers check this public package
directly. It is separate from the original run manifests retained above.
The verifier uses that manifest and checks the published interoperability
artifacts. See [the verification guide](HOW-TO-VERIFY.md) for usage.

| Added file | SHA-256 |
|---|---|
| `verify-evidence.py` | `59c667a5b8819b1682f11626f2f9c12cd998c3cb65c04de091ef629a0e23e11e` |
| `ARTIFACT-SHA256SUMS` | `fbbc21666070ad97cba4b3d22ec33fd1726c472d5c38ae9f426fa7d40979e29e` |

## Migration evidence scope correction, September 15, 2026

The README and migration scope now distinguish the historical managed-copy
100,000-record transformation from the separate live-migration serving API.
The historical concurrent-read probe served zero reads. The earlier wording
that attached read availability to that measured run has been corrected.
The current live 100,000-record availability campaign is explicitly pending;
its eventual result will have its own source, executable, workload and ledger.
Supported live API behavior remains described separately from measured results.

All raw artifacts, vectors, transcripts, reports and verification programs remain
byte-identical to baseline `606a49f560d7b25ec8147cd110e0139ca02d5ba0`.
This editorial note is excluded from its own hash table.

| File | Before correction SHA-256 | After correction SHA-256 |
|---|---|---|
| `README.md` | `d435e7c33634bada1d800272d32000f40b9b909c57a2b9f7ab89632bbe9c0f67` | `bd650ff669d0c3a1763c749ec639934de89613a08edf6c3455d5283988971ecd` |
| `CLAIMS-AND-SCOPE.md` | `36e0e2e911da919b1fa69813f3ca08583c486814c7a5a92211863545f20bdf06` | `11aeed8915dc699116622f27030bb86ae7ae2853851858898cf7ac92fced7958` |

## Four-CPU migration additions, September 16, 2026

The [dated supplement](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/)
adds four original runs and preserves the earlier R5/R6 raw evidence. Its
[provenance mapping](migration/live-hkdf-2026-09-15/four-cpu-2026-09-16/ORIGINAL-PROVENANCE.json)
records byte-identical original/public hashes for every copied file. No
redactions were needed in those selected synthetic auditor inputs, reports or
small custody records. The engine source, executable and database files remain
outside this public package.

Per-run host summaries select the recorded resource limits and terminal state;
the full original inspection is identified by hash. The unchanged Rust auditor,
31 controls and vendored dependencies are reused. The package replay script now
requires all six original verdicts and reports to reproduce, including every
rejection. Dated additions to the README, migration scope and verification guide
point to the new outcomes without replacing the historical measurements.

## Protected-node checkpoint, September 16, 2026, 13:32 UTC

The dated `RELEASE-CHECKPOINT-1332Z.md` and current qualification status summarize
the completed AES-GCM job on source `9467eb1d2cb9aea2625f3e6b6f8d189d447c4172`
and identify the corrected successor. They record original receipt, executable,
inventory and independent-review hashes without publishing private engine
source, binaries, database files, credentials or internal filesystem paths.
Existing raw artifacts, manifests, verifiers and historical snapshots are
unchanged. The checkpoint distinguishes internal execution evidence from the
publicly replayable packages already in this repository.

## Delivery checkpoint, September 16, 2026, 14:38 UTC

`DELIVERY-CHECKPOINT-1430Z.md` and the current qualification status add summaries
of the corrected default release and independently reviewed local delivery
rehearsals. They retain distinct source, executable, original-receipt and review
identities. Retry observations, the original clock-test failure and the wrapper
README provenance exception remain explicit. Private engine source, binaries,
test keys, database files and internal paths are excluded. Existing raw public
artifacts, manifests, verifiers and historical checkpoints are unchanged.

## Native and release checkpoint, September 16, 2026, 15:34 UTC

`NATIVE-CHECKPOINT-1529Z.md` and the current qualification status summarize the
corrected candidate's completed default native role. Original source, executable,
receipt, inventory and independent-review identities are retained in the public
summary. The initial review-adapter refusal and the distinct temporal and
snapshot evidence scopes remain explicit. Private source, executables, database
files, synthetic test keys and internal paths are excluded. Existing raw public
artifacts and replay tools are unchanged.

The same checkpoint adds the corrected AES-GCM release job and its original
receipt and artifact-membership review. It records the skipped evaluator-archive
step and keeps the release-executable fault supplement open.

## Worker and delivery checkpoint, September 16, 2026, 16:06 UTC

`WORKER-AND-DELIVERY-CHECKPOINT-1556Z.md`, the README and current qualification
status summarize the separately reviewed corrected-release worker supplement
and optimized local 16,512-record rehearsal. Source, executable, profile, result
and review identities remain distinct. The two short-mission refusals and the
unstarted 24-hour mission are recorded as observed at this timestamp. Private
executables, stores, synthetic keys and internal paths are excluded. Existing
raw artifacts, manifests, replay tools and dated checkpoints remain unchanged.
