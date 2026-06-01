use crate::conf::{CmdOptConf, Color, EnvConf};
use crate::util::err::BrokenPipeError;
use regex::Regex;
use runnel::RunnelIoe;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, env: &EnvConf) -> anyhow::Result<()> {
    let re = Regex::new(&conf.opt_exp)?;
    //
    let r = do_match_proc(sioe, conf, env, &re);
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

#[derive(Debug)]
struct CurrColor {
    curr_color: Color,
}
impl CurrColor {
    fn new() -> Self {
        Self {
            curr_color: Color::None,
        }
    }
    #[allow(dead_code)]
    fn curr_color(&self) -> Color {
        self.curr_color
    }
    fn next_color(&mut self) -> Color {
        let c_color = match self.curr_color {
            Color::None => Color::Red,
            Color::Red => Color::Green,
            Color::Green => Color::Blue,
            Color::Blue => Color::Cyan,
            Color::Cyan => Color::Magenta,
            Color::Magenta => Color::Yellow,
            Color::Yellow => Color::Red,
        };
        self.curr_color = c_color;
        self.curr_color
    }
}

#[derive(Debug)]
struct ColorSeq<'a> {
    env: &'a EnvConf,
}
impl<'a> ColorSeq<'a> {
    fn new(env: &'a EnvConf) -> Self {
        Self { env }
    }
    fn color_seq_start(&self, color: Color) -> &str {
        match color {
            Color::None => "",
            Color::Red => self.env.color_seq_red_start.as_str(),
            Color::Green => self.env.color_seq_green_start.as_str(),
            Color::Blue => self.env.color_seq_blue_start.as_str(),
            Color::Cyan => self.env.color_seq_cyan_start.as_str(),
            Color::Magenta => self.env.color_seq_magenta_start.as_str(),
            Color::Yellow => self.env.color_seq_yellow_start.as_str(),
        }
    }
    fn color_seq_end(&self) -> &str {
        self.env.color_seq_end.as_str()
    }
}

#[derive(Debug)]
struct MarkColorLNum {
    mark: String,
    color: Color,
    lnum: usize,
}

#[derive(Debug, Clone, Copy)]
struct ColoredRange {
    st: usize,
    ed: usize,
    color: Color,
}

fn clean_cycle_vec(limit_num: usize, line_num: usize, v: &mut Vec<MarkColorLNum>) {
    v.retain(|c| line_num - c.lnum <= limit_num);
}

fn do_match_proc(
    sioe: &RunnelIoe,
    _conf: &CmdOptConf,
    env: &EnvConf,
    re: &Regex,
) -> anyhow::Result<()> {
    let mut curr_color = CurrColor::new();
    let color_seq = ColorSeq::new(env);
    let mut cycle_vec: Vec<MarkColorLNum> = Vec::new();
    //let mut curr_color: Color = Color::None;
    let mut line_num: usize = 0;
    //
    for line in sioe.pg_in().lines() {
        let line_s = line?;
        let line_ss = line_s.as_str();
        line_num += 1;
        //
        let (b_found, colored_ranges) =
            make_line_color_mark(re, &mut cycle_vec, &mut curr_color, line_num, line_ss)?;
        if b_found {
            let out_s = make_out_s(&color_seq, line_ss, &colored_ranges)?;
            sioe.pg_out().write_line(out_s)?;
        } else {
            sioe.pg_out().write_line(line_s)?;
        }
        if line_num % 30 == 0 {
            clean_cycle_vec(50, line_num, &mut cycle_vec);
        }
    }
    //
    sioe.pg_out().flush_line()?;
    //
    Ok(())
}

fn make_line_color_mark(
    re: &Regex,
    cycle_vec: &mut Vec<MarkColorLNum>,
    curr_color: &mut CurrColor,
    line_num: usize,
    line_ss: &str,
) -> anyhow::Result<(bool, Vec<ColoredRange>)> {
    let mut colored_ranges: Vec<ColoredRange> = Vec::new();
    let mut b_found = false;
    //
    for cap in re.captures_iter(line_ss) {
        b_found = true;
        //
        // If the regex has at least one capture group (len > 1), use the first capture group (index 1)
        // for coloring. Otherwise, default to the entire match (index 0).
        let group_index = if cap.len() > 1 { 1 } else { 0 };

        let (st, ed): (usize, usize) = match cap.get(group_index) {
            Some(m) => (m.start(), m.end()),
            None => (0, 0),
        };
        if st == ed {
            continue;
        }
        let mark_s = &line_ss[st..ed];
        let pos = cycle_vec.iter().position(|c| c.mark == mark_s);
        let color = match pos {
            Some(p) => {
                cycle_vec[p].lnum = line_num;
                cycle_vec[p].color
            }
            None => {
                let c_color = curr_color.next_color();
                cycle_vec.push(MarkColorLNum {
                    mark: mark_s.to_string(),
                    color: c_color,
                    lnum: line_num,
                });
                c_color
            }
        };
        colored_ranges.push(ColoredRange { st, ed, color });
    }
    Ok((b_found, colored_ranges))
}

fn make_out_s(
    color_seq: &ColorSeq,
    line_ss: &str,
    colored_ranges: &[ColoredRange],
) -> anyhow::Result<String> {
    let color_end_s = color_seq.color_seq_end();
    //
    let mut out_s: String = String::new();
    let mut last_pos: usize = 0;
    for range in colored_ranges {
        if range.st > last_pos {
            out_s.push_str(&line_ss[last_pos..range.st]);
        }
        let color_start_s = color_seq.color_seq_start(range.color);
        out_s.push_str(color_start_s);
        out_s.push_str(&line_ss[range.st..range.ed]);
        out_s.push_str(color_end_s);
        last_pos = range.ed;
    }
    if last_pos < line_ss.len() {
        out_s.push_str(&line_ss[last_pos..]);
    }
    Ok(out_s)
}

#[cfg(test)]
mod debug {
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<super::MarkColorLNum>(), 40);
    }
}
