yosys -import

read_verilog -sv src/processor/core/decoder/* src/processor/core/rf/* src/processor/core/ieu/* src/processor/core/ifu/* src/processor/core/lsu/* src/processor/core/core.sv 

hierarchy -top core
write_json core.json
