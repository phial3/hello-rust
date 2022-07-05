///////// third lib /////////////
struct ThirdPartPage<'a> {
    name: &'a str,
    content: &'a str
}

struct ThirdPartDoc<'a> {
    page: ThirdPartPage<'a>
}

impl <'a> ThirdPartDoc<'a> {
    fn get_page(&self) -> &ThirdPartPage<'a> {
        &self.page
    }
}

///////// resource //////////


trait ResourceTrait<'a> {
    fn get_content(&self) -> &str;
}

struct TirdpartResource<'a> {
    data: &'a ThirdPartPage<'a>
}

impl<'a> ResourceTrait<'a> for TirdpartResource<'a> {
    fn get_content(&self) -> &str {
        self.data.content
    }
}

////////// main logic ////////

struct Owner<'a> {
    doc: Option<ThirdPartDoc<'a>>
}

impl<'a> Owner<'a> {
    fn get_resource(&'a self) -> Option<Box<dyn ResourceTrait<'a> + 'a>> {
        match &self.doc {
            Some(doc) => {
                Some(Box::new(TirdpartResource {
                    data: doc.get_page()
                }))
            },
            None => Option::None
        }
    }
}

fn main() {
    let owner = Owner {
        doc: Some(ThirdPartDoc {
            page: ThirdPartPage {
                name: "page",
                content: "content"
            }
        })
    };

    let res = owner.get_resource();

    println!("{}", res.unwrap().get_content());
}