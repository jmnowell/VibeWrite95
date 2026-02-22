#[derive(Debug, Clone, PartialEq)]
pub enum DiffLine {
    Same(String),
    Added(String),
    Removed(String),
}

pub fn diff_lines(current: &str, snapshot: &str) -> Vec<(DiffLine, DiffLine)> {
    let current_lines: Vec<&str> = current.lines().collect();
    let snapshot_lines: Vec<&str> = snapshot.lines().collect();

    let lcs = compute_lcs(&current_lines, &snapshot_lines);

    let mut result = Vec::new();
    let mut ci = 0;
    let mut si = 0;

    for (cl, sl) in lcs {
        // Lines before LCS match - removed from current, added in snapshot
        while ci < cl {
            let cur_line = DiffLine::Removed(current_lines[ci].to_string());
            let snap_line = if si < sl {
                let l = DiffLine::Added(snapshot_lines[si].to_string());
                si += 1;
                l
            } else {
                DiffLine::Added(String::new())
            };
            result.push((cur_line, snap_line));
            ci += 1;
        }
        while si < sl {
            result.push((
                DiffLine::Removed(String::new()),
                DiffLine::Added(snapshot_lines[si].to_string()),
            ));
            si += 1;
        }
        result.push((
            DiffLine::Same(current_lines[ci].to_string()),
            DiffLine::Same(snapshot_lines[si].to_string()),
        ));
        ci += 1;
        si += 1;
    }

    // Remaining lines
    while ci < current_lines.len() || si < snapshot_lines.len() {
        let cur = if ci < current_lines.len() {
            let l = DiffLine::Removed(current_lines[ci].to_string());
            ci += 1;
            l
        } else {
            DiffLine::Removed(String::new())
        };
        let snap = if si < snapshot_lines.len() {
            let l = DiffLine::Added(snapshot_lines[si].to_string());
            si += 1;
            l
        } else {
            DiffLine::Added(String::new())
        };
        result.push((cur, snap));
    }

    result
}

fn compute_lcs(a: &[&str], b: &[&str]) -> Vec<(usize, usize)> {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut lcs = Vec::new();
    let mut i = m;
    let mut j = n;
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            lcs.push((i - 1, j - 1));
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    lcs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_lines() {
        let current = "line1\nchanged line\nline3";
        let snapshot = "line1\noriginal line\nline3";
        let diff = diff_lines(current, snapshot);
        assert!(!diff.is_empty());
    }
}
