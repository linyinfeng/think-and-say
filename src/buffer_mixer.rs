use crate::print_buffer::PrintBuffer;
use crate::print_buffer::PrintLocation;

#[derive(Debug, Clone)]
pub struct MixerLayer<'a> {
    pub line_offset: isize,
    pub column_offset: isize,
    pub buffer: PrintBuffer<'a>,
}

#[derive(Debug, Clone)]
pub struct BufferMixer<'a> {
    pub top: isize,                  // inclusive
    pub left: isize,                 // inclusive
    pub bottom: isize,               // exclusive
    pub right: isize,                // exclusive
    pub layers: Vec<MixerLayer<'a>>, // front is bottom
}

impl<'a> Default for BufferMixer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BufferMixer<'a> {
    pub fn new() -> Self {
        BufferMixer {
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: MixerLayer<'a>) {
        let layer_height = layer.buffer.lines_len();
        let layer_width = layer.buffer.columns_len();

        let layer_top = layer.line_offset;
        let layer_left = layer.column_offset;
        let layer_bottom = layer_top + layer_height as isize;
        let layer_right = layer_left + layer_width as isize;

        self.top = self.top.min(layer_top);
        self.left = self.left.min(layer_left);
        self.bottom = self.bottom.max(layer_bottom);
        self.right = self.right.max(layer_right);

        self.layers.push(layer);
    }

    pub fn mix(&self) -> MixerLayer<'a> {
        let mut target = MixerLayer {
            line_offset: self.top,
            column_offset: self.left,
            buffer: Default::default(),
        };
        for layer in &self.layers {
            for (source_line_index, source_line) in layer.buffer.lines.iter().enumerate() {
                for (source_column_index, source_print) in source_line.iter().enumerate() {
                    if let PrintLocation::Grapheme(g) = source_print {
                        let target_line_index =
                            -target.line_offset + layer.line_offset + source_line_index as isize;
                        let target_column_index = -target.column_offset
                            + layer.column_offset
                            + source_column_index as isize;
                        assert!(target_line_index >= 0);
                        assert!(target_column_index >= 0);
                        let target_line_index = target_line_index as usize;
                        let target_column_index = target_column_index as usize;
                        log::info!("set target buffer grapheme: line_index = {}, column_index = {}, grapheme = {}",
                                target_line_index, target_column_index, g);
                        target
                            .buffer
                            .set_grapheme(target_line_index, target_column_index, g);
                    }
                }
            }
        }
        target
    }
}
