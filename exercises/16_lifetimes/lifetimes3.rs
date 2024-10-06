// Lifetimes are also needed when structs hold references.

// 结构体与其引用字段的生命周期

struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book: Book<'_> = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
