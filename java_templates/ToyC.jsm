;>> java_templates/ToyC.class <<
;
; Output created by D-Java (Apr  9 2024)
; mailto:umsilve1@cc.umanitoba.ca
; Copyright (c) 1996-1997 Shawn Silverman
;

;Classfile version:
;    Major: 61
;    Minor: 0

.source ToyC.java
.class  public ToyC
; ACC_SUPER bit is set
.super  java/lang/Object

; >> METHOD 1 <<
.method public <init>()V
    .limit stack 1
    .limit locals 1
.line 1
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

; >> METHOD 2 <<
.method public static main([Ljava/lang/String;)V
    .limit stack 1
    .limit locals 1
.line 3
    invokestatic ToyC/toyc_main()I
    pop
.line 4
    return
.end method

; >> METHOD 3 <<
.method static toyc_main()I
    .limit stack 1
    .limit locals 0
.line 7
    iconst_0
    ireturn
.end method
