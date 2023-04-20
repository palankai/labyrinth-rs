use std::collections::HashMap;

pub struct DynamicMap<T: Clone> {
    map: HashMap<i32, HashMap<i32, Vec<T>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DynamicMapEntry<T: Clone> {
    pub x: i32,
    pub y: i32,
    pub elements: Vec<T>,
}

impl<T: Clone> DynamicMap<T> {
    pub fn new() -> Self {
        DynamicMap {
            map: HashMap::new(),
        }
    }

    #[allow(unused)]
    pub fn get(&self, x: i32, y: i32) -> Option<&Vec<T>> {
        if let Some(elements) = self.map.get(&y)?.get(&x) {
            return Some(elements);
        }
        None
    }

    #[allow(unused)]
    pub fn add(&mut self, x: i32, y: i32, element: T) {
        let row = self.map.entry(y).or_insert(HashMap::new());
        let elements = row.entry(x).or_insert(Vec::new());
        elements.push(element);
    }

    pub fn set(&mut self, x: i32, y: i32, element: T) {
        let row = self.map.entry(y).or_insert(HashMap::new());
        let elements = row.entry(x).or_insert(Vec::new());
        if !elements.is_empty() {
            elements.clear();
        }
        elements.push(element);
    }

    #[allow(unused)]
    pub fn clear(&mut self, x: i32, y: i32) {
        if let Some(row) = self.map.get_mut(&y) {
            if let Some(elements) = row.get_mut(&x) {
                elements.clear();
            }
        }
    }

    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        if let Some(row) = self.map.get(&y) {
            if let Some(elements) = row.get(&x) {
                elements.is_empty()
            } else {
                true
            }
        } else {
            true
        }
    }

    pub fn all(&self) -> Vec<DynamicMapEntry<T>> {
        let mut entries: Vec<DynamicMapEntry<T>> = Vec::new();
        for (y, row) in self.map.iter() {
            for (x, elements) in row.iter() {
                entries.push(DynamicMapEntry {
                    x: *x,
                    y: *y,
                    elements: elements.clone(),
                })
            }
        }
        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_all_out_of_empty() {
        let m: DynamicMap<i32> = DynamicMap::new();
        let all = m.all();

        assert!(all.is_empty());
    }

    #[test]
    fn test_empty() {
        let m: DynamicMap<i32> = DynamicMap::new();

        assert!(m.is_empty(0, 0) == true);
    }

    #[test]
    fn test_not_empty() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.add(3, 6, 1);

        assert!(m.is_empty(3, 6) == false);
    }

    #[test]
    fn test_clear_not_empty() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.add(3, 6, 1);
        m.clear(3, 6);

        assert!(m.is_empty(3, 6) == true);
    }

    #[test]
    fn test_clear_empty() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.clear(0, 0);

        assert!(m.is_empty(0, 0) == true);
    }

    #[test]
    fn test_getting_one_out_of_empty() {
        let m: DynamicMap<i32> = DynamicMap::new();

        let result = m.get(3, 4);

        assert!(result.is_none());
    }

    #[test]
    fn test_adding_one_and_getting_out() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.add(3, 4, 32);

        let result = m.get(3, 4);

        let expected: Vec<i32> = vec![32];

        assert!(Some(&expected) == result)
    }

    #[test]
    fn test_adding_a_few_and_get_them_out() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.add(0, 0, 1);
        m.add(0, 1, 2);
        m.add(-1, 0, 3);
        m.add(-1, 0, 1);

        let mut elements: Vec<(i32, i32, i32)> = Vec::new();
        for entry in m.all() {
            for element in entry.elements {
                elements.push((entry.x, entry.y, element));
            }
        }
        elements.sort();
        let expected: Vec<(i32, i32, i32)> = vec![(-1, 0, 1), (-1, 0, 3), (0, 0, 1), (0, 1, 2)];

        assert!(expected == elements);
    }

    #[test]
    fn test_setting_a_few_and_get_them_out() {
        let mut m: DynamicMap<i32> = DynamicMap::new();
        m.set(0, 0, 1);
        m.set(0, 1, 2);
        m.set(-1, 0, 3);
        m.set(-1, 0, 1);

        let mut elements: Vec<(i32, i32, i32)> = Vec::new();
        for entry in m.all() {
            for element in entry.elements {
                elements.push((entry.x, entry.y, element));
            }
        }
        elements.sort();
        let expected: Vec<(i32, i32, i32)> = vec![(-1, 0, 1), (0, 0, 1), (0, 1, 2)];

        assert!(expected == elements);
    }
}
