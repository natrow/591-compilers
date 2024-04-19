; created using EGRE-591 ToyC compiler by Nathan Rowan and Trevin Vaughan

.source code_gen.tc
.class CodeGenTest
.super java/lang/Object

; >> METHOD 0 <<
.method <init>()V
    .limit stack 1
    .limit locals 1
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

.method public static main([Ljava/lang/String;)V
    .limit stack 1
    .limit locals 1
    invokestatic CodeGenTest/toyc_main()I
    pop
    return
.end method

; begin ToyC code generation...

; >> METHOD 2 <<
.method static toyc_main()I
    .limit stack 999
    .limit locals 999
    ldc 10
    dup
    istore_0
    pop
    ldc 5
    dup
    istore_1
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "a is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_0
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc " and b is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_1
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    iload_0
    iload_1
    if_icmple Label_0
    iconst_0
    goto Label_1
Label_0:
    iconst_1
Label_1:
    dup
    istore_2
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "c is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_2
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    iload_1
    iload_0
    if_icmple Label_2
    iconst_0
    goto Label_3
Label_2:
    iconst_1
Label_3:
    dup
    istore_2
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "c is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_2
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    iload_0
    iload_1
    iload_0
    imul
    iadd
    dup
    istore_0
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "now a is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_0
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    ldc 2
    dup
    istore_3
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "now b is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_3
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    iload_0
    iload_3
    ldc 20
    imul
    isub
    dup
    istore_0
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "now a is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_0
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc " and b is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_1
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "input new b: "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    new java/util/Scanner
    dup
    getstatic java/lang/System/in Ljava/io/InputStream;
    invokespecial java/util/Scanner/<init>(Ljava/io/InputStream;)V
    astore_3
    aload_3
    invokevirtual java/util/Scanner/nextInt()I
    istore_1
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "now b is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_1
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    iload_1
    iload_1
    imul
    dup
    istore_1
    pop
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "now b is "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_1
    invokevirtual java/io/PrintStream/print(I)V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "done!"
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    ldc 0
    ireturn
.end method

; end ToyC code generation
