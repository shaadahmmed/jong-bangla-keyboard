//! Following-r attachment and raw-key exceptions shared by verified rules.

// Independently checked following-r prefixes depend on the final raw key,
// including Z in aZ/oZ, x in kx/Ngx/Ngkx, k in Ngk/kk, and h in Ngkh/Ngkkh/NgkSh/kkh/kSh.
// The N in kkhN/kShN/kxN/ShN and m in kkhm/kShm/kxm/Shm share single-r attachment.
// Selected basic consonants and all exact conjunct tables call this helper.
// Do not infer them from copied Bangla.
pub(super) fn consume_following_r_prefix<'a>(
    remaining: &'a str,
    raw_key: char,
    output: &mut String,
) -> &'a str {
    if remaining.starts_with("rr")
        && !["rri", "rry", "rrZ"]
            .iter()
            .any(|prefix| remaining.starts_with(prefix))
    {
        output.push_str("্রর");
        &remaining[2..]
    } else if (matches!(
        raw_key,
        'Y' | 'k'
            | 'g'
            | 'h'
            | 'n'
            | 'N'
            | 'm'
            | 'l'
            | 'd'
            | 'D'
            | 'b'
            | 'v'
            | 't'
            | 'T'
            | 's'
            | 'S'
            | 'p'
            | 'f'
    ) && remaining.starts_with('r')
        && !remaining.starts_with("rr"))
        || (raw_key == 'Z' && (remaining.starts_with("ry") || remaining.starts_with("rZ")))
    {
        // Selected basic spellings and conjuncts attach single r without a joiner.
        // Z only does so before y/Z.
        output.push_str("্র");
        &remaining[1..]
    } else {
        remaining
    }
}
