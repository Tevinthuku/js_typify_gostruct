fn main() -> Result<(), Vec<String>> {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string 
    }

    "#;
    let tokens = js_typify_gostruct::scanner::scan(example)?;
    let parsed_result = js_typify_gostruct::parser::parse(&tokens)?;
    println!("{:?}", parsed_result);

    Ok(())
}
