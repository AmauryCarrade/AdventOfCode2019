use crate::{first_answer, input, second_answer};
use itertools::Itertools;
use std::fmt::{Display, Formatter, Error, Write};

#[derive(Debug)]
struct SpaceImage {
    /// A list of layers.
    /// Each layer is a list of pixels. Each list is width × height in size.
    layers: Vec<Vec<u32>>,

    width: usize,
    height: usize,
}

impl SpaceImage {
    pub fn new(data: &String, width: usize, height: usize) -> Self {
        SpaceImage {
            layers: data
                .chars()
                .filter_map(|num_str| num_str.to_digit(10))
                .chunks(width * height)
                .into_iter()
                .map(|layer| layer.into_iter().collect::<Vec<_>>())
                .collect(),
            width,
            height,
        }
    }

    /// Calculates the Space Image Checksum.
    pub fn checksum(&self) -> usize {
        let (ones, twos): (Vec<u32>, Vec<u32>) = self
            .layers
            .iter()
            .map(|layer_pixels| {
                (
                    layer_pixels.clone(),
                    layer_pixels
                        .into_iter()
                        .filter(|pixel| *pixel == &0)
                        .count(),
                )
            })
            .min_by(|(_, zeros_1), (_, zeros_2)| zeros_1.cmp(zeros_2))
            .unwrap()
            .0
            .into_iter()
            .filter(|n| *n == 1 || *n == 2)
            .partition(|n| *n == 1);

        ones.len() * twos.len()
    }

    pub fn pixels(&self) -> Vec<u32> {
        (0..self.layers.first().unwrap().len())
            .map(|pixel| {
                self.layers
                    .iter()
                    .map(|layer| layer.get(pixel).unwrap_or(&2))
                    .cloned()
                    .fold(2, |final_pix, pix| match final_pix {
                        0 | 1 => final_pix,
                        _ => pix
                    })
            })
            .collect()
    }
}

impl Display for SpaceImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.pixels().iter().chunks(self.width).into_iter().for_each(|line| {
            line.into_iter().map(|pixel| match pixel {
                1 => '×',
                _ => ' ',
            }).for_each(|pixel| f.write_char(pixel).expect("Cannot write space image pixel"));
            f.write_str("\n").expect("Cannot write space image line");
        });

        Ok(())
    }
}

pub fn run() {
    let space_image = SpaceImage::new(input(8).first().unwrap(), 25, 6);

    first_answer("Space Image Checksum", &space_image.checksum());
    second_answer("Space Image", &format!("\n\n{}", space_image));
}
