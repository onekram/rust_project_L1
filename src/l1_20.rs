trait Printer {
    fn print(&self, text: &str);
}

struct OldPrinter;

impl OldPrinter {
    fn print_old(&self, text: &str) {
        println!("Old printer method 'print_old' message: {}", text);
    }
}
struct PrinterAdapter {
    adaptee: OldPrinter,
}

impl Printer for PrinterAdapter {
    fn print(&self, text: &str) {
        self.adaptee.print_old(text);
    }
}

struct NewPrinter;

impl Printer for NewPrinter {
    fn print(&self, text: &str) {
        println!("New printer method 'print' message: {}", text);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_test() {
        let new_printer = NewPrinter;
        new_printer.print("Hello from the new printer!");
    
        let old_printer = OldPrinter;
        let old_printer_adapter = PrinterAdapter { adaptee: old_printer };
        old_printer_adapter.print("Hello from the old printer via adapter!");
    }
}
