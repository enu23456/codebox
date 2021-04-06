 /**
 * @file signed_clip.v
 * @brief クリッピング処理有りの符号付16ビット整数の8ビット幅縮小化処理
 * @note  クリッピング処理とはオーバーorアンダーフローを検出し、出力信号を最大値または最小値に保つこと
 */

/**
 * @brief 16bit の入力に対してクリッピング処理を施して8bitにして出力する
 */
/* ------------------------------------------------------------------------- */
module signed_clip(
    input [15:0] in,
    output [7:0] result,
    output error          // クリッピング処理が発生したかどうか。1で処理が発生
    );

    assign error = ~&in[15:7] & |in[15:7];
    assign result = ~error ? in[7:0] : (in[15] ? 16'h80 : 16'h7F); 
endmodule

/**
 * @brief ビット幅縮小処理のテストベンチ
 */
/* ------------------------------------------------------------------------- */

module signed_clip_tb();
    /* input */
    reg [15:0] in;
    
    /* output */
    wire [7:0] result;
    wire error;

    /* Instaniate the Unit Under Test */
    signed_clip signed_clip(in, result, error);

    /* describe observed signal */
    initial begin
        $monitor("input = %d, result = %d, error = %d", in, result, error);
        // $dumpfile("signed_clip.vcd");
        // $dumpvars(0, signed_clip_tb);
    end

    /* describe test signal */
    parameter STEP = 100;
    initial begin
        in = 16'h0070;
        #STEP
        in = 16'h2000;
        #STEP
        in = 16'h9000;
        #STEP
        $finish;
    end
endmodule
