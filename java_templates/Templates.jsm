;>> Templates.class <<
;
; Output created by D-Java (Apr  9 2024)
; mailto:umsilve1@cc.umanitoba.ca
; Copyright (c) 1996-1997 Shawn Silverman
;

;Classfile version:
;    Major: 61
;    Minor: 0

.source Templates.java
.class  Templates
; ACC_SUPER bit is set
.super  java/lang/Object

; >> METHOD 1 <<
.method <init>()V
    .limit stack 1
    .limit locals 1
.line 3
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

; >> METHOD 2 <<
.method public static main([Ljava/lang/String;)V
    .limit stack 0
    .limit locals 1
.line 5
    return
.end method

; >> METHOD 3 <<
.method public static expressionStatement()V
    .limit stack 2
    .limit locals 3
.line 8
    iconst_3
    istore_0
    iconst_2
    istore_1
.line 10
    iload_0
    iload_1
    if_icmpne Label1
    iconst_1
    goto Label2
Label1:
    iconst_0
Label2:
    istore_2
.line 11
    iload_0
    iload_1
    if_icmpeq Label3
    iconst_1
    goto Label4
Label3:
    iconst_0
Label4:
    istore_2
.line 12
    iload_0
    iload_1
    if_icmpge Label5
    iconst_1
    goto Label6
Label5:
    iconst_0
Label6:
    istore_2
.line 13
    iload_0
    iload_1
    if_icmpgt Label7
    iconst_1
    goto Label8
Label7:
    iconst_0
Label8:
    istore_2
.line 14
    iload_0
    iload_1
    if_icmple Label9
    iconst_1
    goto Label10
Label9:
    iconst_0
Label10:
    istore_2
.line 15
    iload_0
    iload_1
    if_icmplt Label11
    iconst_1
    goto Label12
Label11:
    iconst_0
Label12:
    istore_2
.line 18
    iload_0
    iload_1
    imul
    istore_2
.line 19
    iload_0
    iload_1
    idiv
    istore_2
.line 20
    iload_0
    iload_1
    irem
    istore_2
.line 21
    iload_0
    ifeq Label13
    iload_1
    ifeq Label13
    iconst_1
    goto Label14
Label13:
    iconst_0
Label14:
    istore_2
.line 24
    iload_0
    iload_1
    iadd
    istore_2
.line 25
    iload_0
    iload_1
    isub
    istore_2
.line 26
    iload_0
    ifne Label15
    iload_1
    ifeq Label16
Label15:
    iconst_1
    goto Label17
Label16:
    iconst_0
Label17:
    istore_2
.line 29
    iload_0
    ineg
    istore_2
.line 30
    iload_0
    ifne Label18
    iconst_1
    goto Label19
Label18:
    iconst_0
Label19:
    istore_2
.line 31
    return
.end method

; >> METHOD 4 <<
.method public static ifElseStatement()V
    .limit stack 2
    .limit locals 2
.line 34
    bipush 10
    istore_0
.line 35
    iconst_0
    istore_1
.line 36
    iload_0
    ifle Label1
.line 37
    iload_1
    iconst_3
    iadd
    istore_1
    goto Label2
.line 39
Label1:
    iload_1
    iconst_3
    isub
    istore_1
.line 41
Label2:
    return
.end method

; >> METHOD 5 <<
.method public static returnStatement()I
    .limit stack 2
    .limit locals 1
.line 44
    iconst_0
    istore_0
.line 45
    iload_0
    iconst_5
    if_icmple Label1
.line 46
    iload_0
    ireturn
.line 49
Label1:
    bipush 10
    istore_0
.line 50
    iload_0
    ireturn
.end method

; >> METHOD 6 <<
.method public static whileStatement()V
    .limit stack 2
    .limit locals 1
.line 54
    iconst_0
    istore_0
.line 55
Label1:
    iload_0
    iconst_5
    if_icmpge Label2
.line 56
    iinc 0 1
    goto Label1
.line 58
Label2:
    return
.end method

; >> METHOD 7 <<
.method public static readStatement()V
    .limit stack 3
    .limit locals 2
.line 62
    new java/util/Scanner
    dup
    getstatic java/lang/System/in Ljava/io/InputStream;
    invokespecial java/util/Scanner/<init>(Ljava/io/InputStream;)V
    astore_1
.line 63
    aload_1
    invokevirtual java/util/Scanner/nextInt()I
    istore_0
.line 64
    return
.end method

; >> METHOD 8 <<
.method public static writeStatement()V
    .limit stack 2
    .limit locals 1
.line 67
    ldc "test"
    astore_0
.line 68
    getstatic java/lang/System/out Ljava/io/PrintStream;
    aload_0
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
.line 69
    return
.end method

; >> METHOD 9 <<
.method public static newLineStatement()V
    .limit stack 1
    .limit locals 0
.line 72
    getstatic java/lang/System/out Ljava/io/PrintStream;
    invokevirtual java/io/PrintStream/println()V
.line 73
    return
.end method
