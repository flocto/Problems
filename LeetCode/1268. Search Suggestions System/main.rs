impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut products = products;
        products.sort();
        let mut res = Vec::new();
        let mut index = 0;
        let mut cur = String::new();
        for c in search_word.chars() {
            cur.push(c);
            while(index < products.len() && !products[index].starts_with(&cur)) {
                index += 1;
            }
            let mut tmp = Vec::new();
            for i in index..index+3 {
                if i >= products.len() || !products[i].starts_with(&cur) {
                    break;
                }
                tmp.push(products[i].clone());
            }
            res.push(tmp);
        }
        res
    }
}