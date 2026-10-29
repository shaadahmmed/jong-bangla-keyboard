//! A small, deterministic Bangla phonetic conversion slice.
//!
//! Supports ASCII digits, contextual lowercase `y`/`w`/`x`, exact
//! `kx`/`kkh`/`kSh`/`Ngx`/`Ngk`/`Ngkh`/`Ngkx`/`Ngkkh`/`NgkSh`, uppercase `Z` and `aZ`/`oZ`,
//! exact `kkhN`/`kShN`/`kxN`, `kkhm`/`kShm`/`kxm`, `kk`, `ShN`/`Shm`, `Ngg`/`Nggh`,
//! `gg`/`gN`/`NN`/`hN`/`gn`/`ghn`/`hn`/`dg`/`dgh`/`Rg`,
//! `ngh`/`nn`/`Nn`/`mn`/`hm`/`hl`/`nm`/`Nm`/`mm`/`ml`/`Ngm`/`gm`,
//! `dd`/`ddh`/`dhn`/`dhm`/`dm`/`dbh`/`dv`/`DD`,
//! `nd`/`ndh`/`nD`/`ND`/`NDh`/`nt`/`nth`/`nT`/`NT`/`nTh`/`NTh`,
//! `tt`/`tth`/`tn`/`tm`/`TT`/`Tm`, `kt`/`kT`/`kl`,
//! `nk`/`nkh`/`lk`/`sk`/`skh`/`skl`/`Shk`/`ks` and longer `lkh`/`ksh`,
//! `gl`/`ll`/`bl`/`pl`/`sl`/`shl`/`bhl`/`phl`/`vl`/`fl`/`Sl`,
//! `mb`/`mbh`/`mv`/`mp`/`mph`/`mf` and longer `mpl`, plus `lm`/`sm`/`shm`/`Sm`,
//! lowercase `ng` with selected vowel forms, the consonant sequences below, and
//! `a`/`i`/`I`/`u`/`U`/`e`/`ee`/`o`/`O`/`oo`/`OI`/`OU`/`rri`.
//! Unmatched Unicode characters pass through unchanged. This is not yet a
//! complete Avro-compatible engine: remaining consonants, other conjuncts, case
//! normalization, punctuation rules, and escapes are not implemented.
//!
//! ```
//! use jong_bangla_core::transliterate;
//!
//! assert_eq!(transliterate("kh12T"), "খ১২ট");
//! assert_eq!(transliterate("t T th Th n N"), "ত ট থ ঠ ন ণ");
//! assert_eq!(transliterate("ami khani kia"), "আমি খানি কিয়া");
//! assert_eq!(transliterate("tumi bhUmi kua"), "তুমি ভূমি কুয়া");
//! assert_eq!(transliterate("ki kI nIti"), "কি কী নীতি");
//! assert_eq!(transliterate("ami ke kee"), "আমি কে কী");
//! assert_eq!(transliterate("tumi kemon tOmake"), "তুমি কেমন তোমাকে");
//! assert_eq!(transliterate("OI kOI OU kOU nOUka"), "ঐ কৈ ঔ কৌ নৌকা");
//! assert_eq!(transliterate("rri krri krriti rriN"), "ঋ কৃ কৃতি ঋণ");
//! assert_eq!(transliterate("ram rani rrri rrrri"), "রাম রানি রররি ররৃ");
//! assert_eq!(transliterate("pani phOn fOn"), "পানি ফোন ফোন");
//! assert_eq!(transliterate("cabi chabi jani jhuri"), "চাবি ছাবি জানি ঝুরি");
//! assert_eq!(transliterate("din dhan Dan Dhaka"), "দিন ধান ডান ঢাকা");
//! assert_eq!(transliterate("lal holud bhalO"), "লাল হলুদ ভালো");
//! assert_eq!(transliterate("s sh S Sh"), "স শ শ ষ");
//! assert_eq!(transliterate("sokal shishu SiSu bhaSha"), "সকাল শিশু শিশু ভাষা");
//! assert_eq!(transliterate("baRi gaRi gaRh drriRh"), "বাড়ি গাড়ি গাঢ় দৃঢ়");
//! assert_eq!(transliterate("zai zabe zodi zokhon"), "যাই যাবে যদি যখন");
//! assert_eq!(transliterate("Ng NG aNgul raNga"), "ঙ ঞ আঙুল রাঙা");
//! assert_eq!(transliterate("bangla rong nga ngi"), "বাংলা রং ঙ্গা ঙ্গি");
//! assert_eq!(transliterate("Ngrri NGrrri ngrrrri"), "ঙররি ঞরৃ ংররররি");
//! assert_eq!(transliterate("zaY hoY hoYe maYa"), "যায় হয় হয়ে মায়া");
//! assert_eq!(transliterate("y ay ky kiya"), "ইয় আয় ক্য কিয়া");
//! assert_eq!(transliterate("kZa aZ oZ baZ"), "ক্যা অ্যা অ্য বঅ্যা");
//! assert_eq!(transliterate("yrr yrri yry Zry"), "ইয়্রর ইয়ৃ ইয়র\u{200D}্য ্য্র্য");
//! assert_eq!(transliterate("Yr Yrri Yry Yrry"), "য়্র য়ৃ য়্র্য য়রর\u{200D}্য");
//! assert_eq!(transliterate("q qa qi qOI qotha"), "ক কা কি কৈ কথা");
//! assert_eq!(transliterate("v vi vU valO vUmi"), "ভ ভি ভূ ভালো ভূমি");
//! assert_eq!(transliterate("w wa awa kw kwa"), "ও ওয়া আওা ক্ব ক্বা");
//! assert_eq!(transliterate("wr wrri wrrri wrry"), "ওর ওৃ ও্রররি ওরর\u{200D}্য");
//! assert_eq!(transliterate("x xa axa kx kxa"), "এক্স এক্সা আক্সা ক্ষ ক্ষা");
//! assert_eq!(transliterate("xr xrr xrri xrrri kxrr"), "এক্সর এক্স্রর এক্সৃ এক্স্রররি ক্ষ্রর");
//! assert_eq!(transliterate("Ngx Ngxa Ngxrri Ngxrr"), "ঙ্ষ ঙ্ষা ঙ্ষৃ ঙ্ষ্রর");
//! assert_eq!(transliterate("Ngk Ngka Ngkr Ngkrri"), "ঙ্ক ঙ্কা ঙ্ক্র ঙ্কৃ");
//! assert_eq!(transliterate("Ngkh Ngkha Ngkhr Ngkhrri"), "ঙ্খ ঙ্খা ঙ্খ্র ঙ্খৃ");
//! assert_eq!(transliterate("Ngkx Ngkxa Ngkxrri Ngkxrr"), "ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষৃ ঙ্ক্ষ্রর");
//! assert_eq!(transliterate("Ngkkh Ngkkha Ngkkhr Ngkkhrri"), "ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষ্র ঙ্ক্ষৃ");
//! assert_eq!(transliterate("NgkSh NgkSha NgkShr NgkShrri"), "ঙ্ক্ষ ঙ্ক্ষা ঙ্ক্ষ্র ঙ্ক্ষৃ");
//! assert_eq!(transliterate("kkh kkha kkhr kkhrri"), "ক্ষ ক্ষা ক্ষ্র ক্ষৃ");
//! assert_eq!(transliterate("kSh kSha kShr kShrri"), "ক্ষ ক্ষা ক্ষ্র ক্ষৃ");
//! assert_eq!(transliterate("kkhN kShNa kxNr kxNrri"), "ক্ষ্ণ ক্ষ্ণা ক্ষ্ণ্র ক্ষ্ণৃ");
//! assert_eq!(transliterate("kkhm kShma kxmr kxmrri"), "ক্ষ্ম ক্ষ্মা ক্ষ্ম্র ক্ষ্মৃ");
//! assert_eq!(transliterate("kk kka kkr kkrri kkkh kkx"), "ক্ক ক্কা ক্ক্র ক্কৃ ক্কখ ক্কক্স");
//! assert_eq!(transliterate("ShN Shm ShNr Shmrri kkShN ShNgx"), "ষ্ণ ষ্ম ষ্ণ্র ষ্মৃ ক্কষ্ণ ষ্ণগক্স");
//! assert_eq!(transliterate("kr khr Shr kry krry kkkhr kkShr Ngkkr"), "ক্র খ্র ষ্র ক্র্য করর‍্য ক্কখ্র ক্কষ্র ঙ্কক্র");
//! assert_eq!(transliterate("gr ghr gry ghrry grri ShNgr Shmgr"), "গ্র ঘ্র গ্র্য ঘরর‍্য গৃ ষ্ণগ্র ষ্মগ্র");
//! assert_eq!(transliterate("Ngg Nggh Nggr Ngghr Nggrri Ngghry"), "ঙ্গ ঙ্ঘ ঙ্গ্র ঙ্ঘ্র ঙ্গৃ ঙ্ঘ্র্য");
//! assert_eq!(transliterate("gg gN NN hN gn ghn hn dg dgh Rg"), "জ্ঞ গ্ণ ণ্ণ হ্ণ গ্ন ঘ্ন হ্ন দ্গ দ্ঘ ড়্গ");
//! assert_eq!(transliterate("ngh nn Nn mn hm hl nm Nm mm ml Ngm gm mr lr"), "ঙ্ঘ ন্ন ণ্ন ম্ন হ্ম হ্ল ন্ম ণ্ম ম্ম ম্ল ঙ্ম গ্ম ম্র ল্র");
//! assert_eq!(transliterate("dd ddh dhn dhm dm dbh dv DD dr dhr Dr Dhr br bhr vr"), "দ্দ দ্ধ ধ্ন ধ্ম দ্ম দ্ভ দ্ভ ড্ড দ্র ধ্র ড্র ঢ্র ব্র ভ্র ভ্র");
//! assert_eq!(transliterate("nd ndh nD ND NDh nt nth nT NT nTh NTh tr thr Tr Thr"), "ন্দ ন্ধ ন্ড ণ্ড ণ্ঢ ন্ত ন্থ ন্ট ণ্ট ন্ঠ ণ্ঠ ত্র থ্র ট্র ঠ্র");
//! assert_eq!(transliterate("tt tth tn tm TT Tm uttor utthan rotno atma"), "ত্ত ত্থ ত্ন ত্ম ট্ট ট্ম উত্তর উত্থান রত্ন আত্মা");
//! assert_eq!(transliterate("kt kT kl kn km ktm kTm rokto klanto"), "ক্ত ক্ট ক্ল কন কম ক্তম ক্টম রক্ত ক্লান্ত");
//! assert_eq!(transliterate("nk nkh lk lkh sk skl skh Shk ks ksh sr shr Sr"), "ঙ্ক ঙ্খ ল্ক লখ স্ক স্ক্ল স্খ ষ্ক ক্স কশ স্র শ্র শ্র");
//! assert_eq!(transliterate("gl ll bl pl sl shl bhl phl vl fl Sl pr phr fr"), "গ্ল ল্ল ব্ল প্ল স্ল শ্ল ভ্ল ফ্ল ভ্ল ফ্ল শ্ল প্র ফ্র ফ্র");
//! assert_eq!(transliterate("mb mbh mv mp mph mf mpl lm sm shm Sm mbl mplk smp"), "ম্ব ম্ভ ম্ভ ম্প ম্ফ ম্ফ মপ্ল ল্ম স্ম শ্ম শ্ম ম্বল মপ্লক স্মপ");
//! ```

mod transliteration;

pub use transliteration::transliterate;
