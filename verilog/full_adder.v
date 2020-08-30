/**
 * @file full_adder.v
 * @brief Verilog サンプルプログラム。全加算器
*/

 /**
 * @brief 全加算器
 * @param [in] A  入力
 * @param [in] B  入力
 * @param [in] C_I  入力
 * @param [out] S  和
 * @param [out] C_O  桁上がり
 */
/* ------------------------------------------------------------------------- */
module full_adder(
    input A,
    input B,
    input C_I,
    output S,
    output C_O
    );
    
    wire[1:0] added;
    assign added = A + B + C_I;
    assign S = added[0];
    assign C_O = added[1];
endmodule

/**
 * @brief 全加算器のテストベンチ
 */
/* ------------------------------------------------------------------------- */
module full_adder_tb();
    /* input */
    reg a;
    reg b;
    reg c_i;
    
    /* output */
    wire s;
    wire c_o;

    /* Instaniate the Unit Under Test */
    full_adder full_adder_0(a, b, c_i, s, c_o);

    parameter STEP = 100;
    initial begin
        $monitor("A = %d, B = %d, C_I = %d ... S = %d, C_O = %d", a, b, c_i, s, c_o);
        $dumpfile("full_adder.vcd");
        $dumpvars(0, full_adder_tb);

        a = 0;
        b = 0;
        c_i = 0;
        #STEP
        a = 1;
        #STEP
        a = 0;
        b = 1;
        #STEP
        a = 1;
        b = 0;
        c_i = 1;
        #STEP
        a = 1;
        b = 1;
        c_i = 1;
        #STEP
        $finish;
    end
endmodule