
use proc_macro::TokenStream;
// todo!("clean the code ")

#[proc_macro_attribute]
pub fn authenticate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    
    let mut new_item=TokenStream::new();
    
    let  item=item.to_string();
    println!("item: {:?}", item);
    let mut items=item.split("{").collect::<Vec<&str>>();
    println!("\n count: {:?} \n", items.len() );
    
    let code=r#" 
    let token=req.headers().get("Authorization").unwrap().to_str().unwrap().split(" ").nth(1).unwrap();
    let is_valid=is_valid_token(token);
    if !is_valid {
        return HttpResponse::Unauthorized().json(ResponseModel{status:ResponseStatus::Unauthorized,message:"invalid token",data:()})
    }
    "#;
    
    
    let mut fn_body=items.remove(0).to_string();
    fn_body.push_str(" { ");
    fn_body.push_str(code);
    let all_items=items.join("{");
    fn_body.push_str(&all_items);
    
    println!("items: {:?}", fn_body.clone() );
    
   
    new_item=fn_body.parse::<TokenStream>().unwrap();

    new_item
  
  
  
  
}
