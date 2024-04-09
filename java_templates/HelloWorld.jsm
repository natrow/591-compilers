;>> HelloWorld.class <<
;
; Output created by D-Java (Apr  9 2024)
; mailto:umsilve1@cc.umanitoba.ca
; Copyright (c) 1996-1997 Shawn Silverman
;

;Classfile version:
;    Major: 61
;    Minor: 0

.source HelloWorld.java
.class  HelloWorld
; ACC_SUPER bit is set
.super  java/lang/Object

; >> METHOD 1 <<
.method <init>()V
    .limit stack 1
    .limit locals 1
.line 1
    aload_0
    invokespecial java/lang/Object/<init>()V
    return
.end method

; >> METHOD 2 <<
.method public static main([Ljava/lang/String;)V
    .limit stack 2
    .limit locals 1
.line 3
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Hello world"
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
.line 4
    return
.end method
