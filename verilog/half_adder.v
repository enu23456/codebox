/**
 * @file half_adder.v
 * @brief Verilog サンプルプログラム。半加算器
*/

/**
 * @brief 半加算器
 * @param [in] A  入力
 * @param [in] B  入力
 * @param [out] S  和
 * @param [out] C  桁上がり
 */
/* ------------------------------------------------------------------------- */
module half_adder(
    input A,
    input B,
    output S,
    output C
    );
    
    assign S = A ^ B;
    assign C = A & B;
endmodule

/**
 * @brief 半加算器のテストベンチ
 */
/* ------------------------------------------------------------------------- */
`timescale 1ns/1ps  // 単位/精度. 精度は粗くしても処理速度に影響はないらしい

module half_adder_tb();

    /* Input */
    reg a;
    reg b;
    
    /* Output */
    wire s;
    wire c;

    /* Instaniate the Unit Under Test */
    half_adder half_adder_0(a, b, s, c);

    parameter STEP = 10;

    initial begin
        $monitor("A = %d, B = %d, S = %d, C = %d", a, b, s, c);
        $dumpfile("half_adder.vcd");
        $dumpvars(0, half_adder_tb);

        a = 0;
        b = 0;
        #STEP
        a = 1;
        #STEP
        a = 0;
        b = 1;
        #STEP
        a = 1;
        #STEP
        $finish;
    end
endmodule