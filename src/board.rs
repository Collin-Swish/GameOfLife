use std::collections::{HashSet, HashMap};
use crate::cell::Cell;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Board {
    map: HashSet<Cell>
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Board {
    fn surrounds(&self, cell: &Cell) -> [Cell; 8] {
        let cells: [Cell; 8] = [
            Cell::new(cell.x - 1, cell.y + 1),
            Cell::new(cell.x - 1, cell.y),
            Cell::new(cell.x - 1, cell.y - 1),
            Cell::new(cell.x, cell.y + 1),
            Cell::new(cell.x, cell.y - 1),
            Cell::new(cell.x + 1, cell.y + 1),
            Cell::new(cell.x + 1, cell.y),
            Cell::new(cell.x + 1, cell.y - 1),
        ];
        return cells;
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn from_string(data: String) -> Self {
        let mut board = Self::default();
        for line in data.lines() {
            let (first, last) = line.split_once(" ").unwrap();
            let first_num: i64 = first.parse().unwrap();
            let second_num: i64 = last.parse().unwrap();
            board.insert(Cell::new(first_num, second_num));
        }
        board
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn default() -> Self {
        Board { map: HashSet::new() }
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn insert(&mut self, cell: Cell) {
        self.map.insert(cell);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn remove(&mut self, cell: &Cell) {
        self.map.remove(cell);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn set(&mut self, x: i32, y: i32) {
        let cell = Cell::new(x as i64, y as i64);
        self.insert(cell);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn unset(&mut self, x: i32, y: i32) {
        let cell = Cell::new(x as i64, y as i64);
        self.remove(&cell);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn toggle(&mut self, x: i32, y: i32) {
        let cell: Cell = Cell::new(x as i64, y as i64);
        if self.map.contains(&cell) {
            self.map.remove(&cell);
            return;
        }
        self.map.insert(cell);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn copy(&self) -> Self {
        self.clone()
    }
    fn surround_data(&self, cell: &Cell, neighbors: Option<[Cell; 8]>) -> (u8, [Cell; 8]) {
        let neighbor_cells = match neighbors {
            Some(cells) => cells,
            None => self.surrounds(cell),
        };
        let mut count: u8 = 0;
        for i in &neighbor_cells {
            if self.map.contains(i) {
                count += 1;
            }
        }
        return (count, neighbor_cells);
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn get_cells(&self) -> Vec<Cell> {
        return Vec::from_iter(self.map.iter().cloned())
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn surround_count(&self, cell: &Cell) -> u8 {
        return self.surround_data(cell, None).0;
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn evaluate(&mut self) {
        let mut status: HashMap<Cell, bool> = HashMap::new();
        for cell in self.map.iter() {
            let (count, neighbors) = self.surround_data(cell, None);
            if count < 2 {
                let _ = status.insert(cell.clone(), false);
            }
            else if count == 2 || count == 3 {
                let _ = status.insert(cell.clone(), true);
            }
            else if count > 3 {
                let _ = status.insert(cell.clone(), false);
            }
            for neighbor in neighbors {
                if !status.contains_key(&neighbor) {
                        let neighbor_count = self.surround_count(&neighbor);
                    if neighbor_count == 3 {
                        let _ = status.insert(neighbor, true);
                    }
                    else {
                        let _ = status.insert(neighbor, false);
                    }
                }
            }
        }
        for s in status {
            let (cell, stat) = s;
            if stat {
                self.map.insert(cell);
            }
            else {
                self.map.remove(&cell);
            }
        }
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn output(&self, x_size_top: i64, x_size_bot: i64, y_size_top: i64, y_size_bot: i64) {
        print!("{}[2J", 27 as char);
        for y in (y_size_bot..y_size_top).rev() {
            print!("|");
            for x in x_size_bot..x_size_top {
                let cell = Cell::new(x, y);
                if self.map.contains(&cell) {
                    print!("A");
                }
                else {
                    print!(" ");
                }
            }
            print!("|");
            print!("\n");
        }
    }
}

impl Board {
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Cell> {
        self.map.iter()
    }
}