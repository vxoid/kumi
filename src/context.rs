use crate::symbol_table::SymbolTable;

#[derive(Clone)]
pub struct Context<'c> {
    pos: usize,
    name: String,
    parent: Option<&'c Self>,
    symbol_table: SymbolTable<'c>
}

impl<'c> Context<'c> {
    pub fn new(name: &str, parent: Option<&'c Self>, pos: usize) -> Self {
        let symbol_table = parent.map_or(SymbolTable::default(), |parent| SymbolTable::new(parent.get_symbol_table()));
        Self { pos, name: name.to_string(), parent, symbol_table }
    }

    pub fn step(self) -> Option<&'c Self> {
        self.parent
    }

    pub fn generate_traceback(&self) -> String {
        let mut traceback = String::new();
        
        let mut ctx = Some(self.clone());
        while let Some(context) = ctx {
            traceback.push_str(&format!("\tLine {}, in {}\n", context.pos, context.name));

            ctx = context.step().map(|value| value.clone());
        }

        traceback
    }

    pub fn get_symbol_table_mut(&mut self) -> &mut SymbolTable<'c> {
        &mut self.symbol_table
    }

    pub fn get_symbol_table(&self) -> &SymbolTable<'c> {
        &self.symbol_table
    }
}