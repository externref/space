fish_add_path target/debug/

function format
    cargo fmt -p space_cli
    cargo fmt -p space_lib
end

function build 
    cargo build -p $argv[1]
end

