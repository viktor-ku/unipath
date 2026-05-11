#![allow(dead_code)]
#![allow(unused)]

/// Packed path token.
///
/// Most variants are common path component names worth special-casing.
/// `Inline8` and `Inline16` are opcodes for custom UTF-8 components.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    //
    // Administrative path tokens
    //
    //
    /// `\0`
    Null,

    /// `.`
    Dot1,

    /// `..`
    Dot2,

    /// Will resolve to `/home/<username>`
    /// after user provides the username.
    UserHome,

    /// Will resolve to `/home/<username>/.config`
    /// after user provides the username.
    UserConfig,

    //
    // Common 1st level Unix/FHS directory names
    //
    //
    /// `/home`
    Home,

    /// `/root`
    Root,

    /// `/bin`
    Bin,

    /// `/boot`
    Boot,

    /// `/dev`
    Dev,

    /// `/etc`
    Etc,

    /// `/lib`
    Lib,

    /// `/lib64`
    Lib64,

    /// `/media`
    Media,

    /// `/mnt`
    Mnt,

    /// `/opt`
    Opt,

    /// `/proc`
    Proc,

    /// `/run`
    Run,

    /// `/sbin`
    Sbin,

    /// `/srv`
    Srv,

    /// `/sys`
    Sys,

    /// `/tmp`
    Tmp,

    /// `/usr`
    Usr,

    /// `/var`
    Var,

    /// `/app`
    App,

    //
    // Common lower-level Unix/FHS directory names
    //
    //
    /// `share`
    Share,

    /// `local`
    Local,

    /// `include`
    Include,

    /// `src`
    Src,

    /// `log`
    Log,

    /// `cache`
    Cache,

    /// `spool`
    Spool,

    /// `libexec`
    Libexec,

    /// `man`
    Man,

    /// `doc`
    Doc,

    /// `info`
    Info,

    /// `lock`
    Lock,

    /// `mail`
    Mail,

    /// `empty`
    Empty,

    /// `db`
    Db,

    /// `state`
    State,

    //
    // Common XDG/home directory names
    //
    //
    /// `.config`
    DotConfig,

    /// `.cache`
    DotCache,

    /// `.local`
    DotLocal,

    /// `.ssh`
    DotSsh,

    /// `.gnupg`
    DotGnupg,

    /// `Desktop`
    Desktop,

    /// `Documents`
    Documents,

    /// `Downloads`
    Downloads,

    /// `Music`
    Music,

    /// `Pictures`
    Pictures,

    /// `Public`
    Public,

    /// `Templates`
    Templates,

    /// `Videos`
    Videos,

    //
    // Very common developer/tooling directory names
    //
    //
    /// `.git`
    DotGit,

    /// `.cargo`
    DotCargo,

    /// `.rustup`
    DotRustup,

    /// `.npm`
    DotNpm,

    /// `.docker`
    DotDocker,

    /// `.vim`
    DotVim,

    /// `.emacs.d`
    DotEmacsD,

    /// `nvim`
    Nvim,

    /// `node_modules`
    NodeModules,

    /// `target`
    Target,

    //
    // Very common Python directory names
    //
    //
    /// `__pycache__`
    DunderPycache,

    /// `.venv`
    DotVenv,

    /// `venv`
    Venv,

    /// `site-packages`
    SitePackages,

    /// `dist-packages`
    DistPackages,

    /// `.pytest_cache`
    DotPytestCache,

    /// `.mypy_cache`
    DotMypyCache,

    //
    // Inline UTF-8 component opcodes
    //
    //
    /// Custom UTF-8 component with a following `u8` byte length.
    Inline8 = 254,

    /// Custom UTF-8 component with a following `u16` byte length.
    Inline16 = 255,
}

impl Token {
    #[inline]
    pub const fn byte(self) -> u8 {
        self as u8
    }
}

impl From<Token> for u8 {
    #[inline]
    fn from(token: Token) -> Self {
        token as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn token_is_one_byte() {
        assert_eq!(std::mem::size_of::<Token>(), 1);
    }

    #[test]
    fn compose_packed_user_config_nvim_path() {
        let username = "victoria";

        let mut v: Vec<u8> = vec![
            Token::Home.byte(),
            Token::Inline8.byte(),
            username.len() as u8,
        ];

        v.extend_from_slice(username.as_bytes());

        v.extend([
            Token::Null.byte(),
            Token::DotConfig.byte(),
            Token::Nvim.byte(),
        ]);

        assert_eq!(
            v,
            vec![
                Token::Home.byte(),
                Token::Inline8.byte(),
                8,
                b'v',
                b'i',
                b'c',
                b't',
                b'o',
                b'r',
                b'i',
                b'a',
                Token::Null.byte(),
                Token::DotConfig.byte(),
                Token::Nvim.byte(),
            ],
        );
    }
}
