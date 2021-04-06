/**
 * @file adder4.v
 * @brief 4bit加算器
 */

/**
 * @brief 4bit加算器
 */
/* ------------------------------------------------------------------------- */
module adder4 (
    input[3:0] in_1,  // 入力値
    input[3:0] in_2,  // 入力値
    output[3:0] out,  // 加算結果
    output     carry  // 桁上がり
    );

    wire[4:0] result;
    
    assign result = in_1 + in_2;
    assign carry = result[4];
    assign out = result[3:0];
endmodule

/**
 * @brief testmoduleのテストベンチ
 * @note  注記すべき事項を記載
 */
/* ------------------------------------------------------------------------- */
module testmodule_tb();
    
    /* input */
    reg[3:0] in_1;
    reg[3:0] in_2;

    /* output */
    wire[3:0] out;
    wire carry;

    /* Instaniate the Unit Under Test */
    adder4 adder4(in_1, in_2, out, carry);
    
    /* describe observed signal */
    initial begin
        $monitor("in_1 = %b, in_2 = %b ... out = %b, carry = %b", in_1, in_2, out, carry);
        // $dumpfile("adder4.vcd");
        // $dumpvars(0, adder4);
    end

    /* describe test signal */
    parameter STEP = 100;
    initial begin
        in_1 = 4'b1000;
        in_2 = 4'b0100;
        #STEP
        in_1 = 4'b1100;
        in_2 = 4'b0110;
        #STEP
        in_1 = 4'b1111;
        in_2 = 4'b1111;
        #STEP
        $finish;
    end
endmodule
