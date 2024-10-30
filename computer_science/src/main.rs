use algorithm::binary_search;

mod algorithm;
mod data_structure;

fn main() {
    let search_message = match binary_search(vec![1, 3, 5], 5) {
        Ok(index) => &*format!("Searched value index: {index}"),
        Err(_) => "Searched value not found",
    };

    println!("{}", search_message);
}
