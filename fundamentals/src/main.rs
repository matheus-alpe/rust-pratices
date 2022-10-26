mod concepts1;
mod concepts2;

fn main() {
    let concept_number = 2;

    if concept_number == 1 {
        concepts1::variables_and_mutability::example();
        concepts1::data_types::example();
        concepts1::functions::example();
        concepts1::comments::example();
        concepts1::control_flow::example();
    }

    if concept_number == 2 {
        concepts2::ownership::example();
    }
}