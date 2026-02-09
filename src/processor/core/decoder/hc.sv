// Hazard controller
module hc (
    input logic [4:0] rs1_addr_D,
    input logic [4:0] rs2_addr_D,
    input logic [4:0] rd_addr [2:0],

    output logic stall_D
);
    always_comb begin
        stall_D = 0;
        for(int i = 0; i < 3; i++) begin
            if((rs1_addr_D == rd_addr[i] | rs2_addr_D == rd_addr[i]) & rd_addr[i] != 0) begin
                stall_D = 1;
            end
        end
    end
endmodule
