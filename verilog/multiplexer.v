/**
 * @file multiplexer.v
 * @brief Verilogサンプルプログラム。マルチプレクサ
 */

/**
 * @brief マルチプレクサ
 * @parma input[3:0] A
 * @parma input[1:0] S
 * @parma output F
 */
module multiplexer(
    input[3:0] A,
    input[1:0] S,
    output F
    );

    function[1:0] multi;
        input S;
        case(S)
        2'b00:
            multi = A[0];
        2'b01:
            multi = A[1];
        2'b10:
            multi = A[2];
        2'b11:
            multi = A[3];
        default:
            multi = A[3];
        endcase
    endfunction

    assign F = multi(S);

endmodule

/**
 * @brief マルチプレクサのテストベンチ
 */
module multiplexer_test();
    /* input */
    reg[3:0] a;
    reg[1:0] s;

    /* output */
    wire f;

    /* Instaniate the Unit Under Test */
    multiplexer multiplexer(a, s, f);

    parameter STEP = 100;
    initial begin
        $monitor("A = %b, S = %b ... F = %b", a, s, f);
        $dumpfile("multiplexer.vcd");
        $dumpvars(0, multiplexer_test);

        a = 4'b0101;
        s = 2'b00;
        #STEP
        s = 2'b01;
        #STEP
        s = 2'b10;
        #STEP
        s = 2'b10;
        #STEP
        s = 2'b11;
        #STEP
        $finish;
    end
endmodule