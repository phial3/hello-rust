use sqlparser::ast::Statement;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

fn main() {
    // let sql = "SELECT a, b, 123, myfunc(b) \
    //        FROM table_1 \
    //        WHERE a > b AND b < 100 \
    //        ORDER BY a DESC, b";

    // let sql = "update table_2 set a = 1";

    let sql = "Delete from table_3 where id = 1";

    let dialect = GenericDialect {}; // or AnsiDialect, or your own dialect ...

    let parse = Parser::parse_sql(&dialect, sql).unwrap();

    for ast in &parse {
        println!("AST: {:?}", ast);
        match ast {
            Statement::Query(a) => {
                println!("Query AST order by:{:?}", (&a).order_by);
            }
            Statement::Update {
                table,
                assignments,
                from,
                selection,
            } => {
                println!(
                    "Update AST: table-{:?}, assignments-{:?}, from-{:?}, selection-{:?}",
                    table,
                    assignments,
                    from,
                    selection.is_none()
                );
            }
            Statement::Delete {
                table_name,
                selection,
            } => {
                // println!(
                //     "Delete AST: table-{:?}, selection-{:?}",
                //     table_name, match selection {
                //         Some(T) => T,
                //         _ => Expr::IsNull(),
                //     }
                // );
                println!(
                    "Delete AST: table-{:?}, selection-{:?}",
                    table_name, selection.is_none()
                );
            }
            _ => println!("Other ast"),
        }
    }
}
