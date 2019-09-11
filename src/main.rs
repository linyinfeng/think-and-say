#[macro_use]
mod log_dbg;
mod buffer_mixer;
mod print_buffer;

use buffer_mixer::BufferMixer;
use buffer_mixer::MixerLayer;
use print_buffer::PrintBuffer;
use structopt::StructOpt;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
struct Opt {
    /// text to be said
    #[structopt()]
    text: String,

    #[structopt(long, default_value = "4")]
    aspect_ratio: f64,

    #[structopt(long, default_value = "1")]
    min_vertical_padding: usize,

    #[structopt(long, default_value = "2")]
    min_horizontal_padding: usize,
}

fn main() {
    env_logger::init();
    let mut opt = Opt::from_args();

    // ensure every line ends with a '\n'
    let last_char = opt.text.chars().last();
    match last_char {
        Some('\n') => (),
        Some(_) => opt.text.push('\n'),
        None => (),
    }

    // art properties
    let art = include_str!("think-and-say.txt");
    let left_border = 29isize;
    let bottom_border = 9isize;
    let min_area_height = 6;
    let min_area_width = 19;
    let art_print_buffer = log_dbg!(PrintBuffer::from_str(art));

    // text properties
    let text_total_display_width = UnicodeWidthStr::width(opt.text.as_str());
    let scale = (text_total_display_width as f64 / opt.aspect_ratio).sqrt();
    let estimate_width = (scale * opt.aspect_ratio).ceil() as usize;
    let grapheme_max_width = UnicodeSegmentation::graphemes(opt.text.as_str(), true)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let text_area_max_width = estimate_width.max(grapheme_max_width);

    let mut text_line_widths = Vec::new();
    let mut text_line_strs = Vec::new();
    {
        let mut text_lines = 0usize;
        let mut line_remain_display_width = 0usize;
        let mut current_line = Vec::new();
        let mut current_line_width = 0usize;

        for grapheme in UnicodeSegmentation::graphemes(opt.text.as_str(), true) {
            let w = UnicodeWidthStr::width(grapheme);
            // every line ends with a '\n'
            if w > line_remain_display_width || grapheme == "\n" {
                if text_lines != 0 {
                    text_line_widths.push(current_line_width);
                    current_line_width = 0;
                    text_line_strs.push(current_line.concat());
                    current_line.clear()
                }

                // not fit into current line or is a newline control character
                text_lines += 1;
                line_remain_display_width = text_area_max_width; // then the grapheme must fit
            }
            // add into current line
            if grapheme != "\n" {
                current_line.push(grapheme);
                current_line_width += w;
                line_remain_display_width -= w;
            }
        }
    }

    let text_width = log_dbg!(text_line_widths.iter().max().cloned().unwrap_or(0));
    let text_height = log_dbg!(text_line_strs.len());
    let text_arranged = text_line_strs.join("\n");
    let text_print_buffer = PrintBuffer::from_str(&text_arranged);

    let area_width = log_dbg!(min_area_width.max(text_width + opt.min_horizontal_padding * 2));
    let area_height = log_dbg!(min_area_height.max(text_height + opt.min_vertical_padding * 2));

    // create border buffer
    let border_print_buffer = {
        let mut buffer = PrintBuffer::new();
        buffer.set_grapheme(0, 0, "+");
        buffer.set_grapheme(area_height + 1, 0, "+");
        buffer.set_grapheme(0, area_width + 1, "+");
        buffer.set_grapheme(area_height + 1, area_width + 1, "+");
        for c in 0..area_width {
            buffer.set_grapheme(0, c + 1, "-");
            buffer.set_grapheme(area_height + 1, c + 1, "-");
        }
        for l in 0..area_height {
            buffer.set_grapheme(l + 1, 0, "|");
            buffer.set_grapheme(l + 1, area_width + 1, "|");
        }
        buffer
    };

    log::info!("about to mix art_print_buffer:\n{}", text_print_buffer);
    log::info!("about to mix art_print_buffer:\n{}", art_print_buffer);
    log::info!("about to mix border_print_buffer:\n{}", border_print_buffer);

    let top_border = log_dbg!(bottom_border - area_height as isize - 1);
    let border_layer = MixerLayer {
        line_offset: top_border,
        column_offset: left_border,
        buffer: border_print_buffer,
    };
    let art_layer = MixerLayer {
        line_offset: 0,
        column_offset: 0,
        buffer: art_print_buffer,
    };
    let text_layer = {
        let line_offset_in_area = (area_height - text_height) / 2;
        let column_offset_in_area = (area_width - text_width) / 2;
        MixerLayer {
            line_offset: top_border + line_offset_in_area as isize + 1,
            column_offset: left_border + column_offset_in_area as isize + 1,
            buffer: text_print_buffer,
        }
    };
    let mixer = {
        let mut mixer = BufferMixer::new();
        mixer.add_layer(border_layer);
        mixer.add_layer(art_layer);
        mixer.add_layer(text_layer);
        mixer
    };
    let result = mixer.mix();
    print!("{}", result.buffer);
}
