00000000000b31c0 <strcmp@@GLIBC_2.2.5>:
   b31c0:       f3 0f 1e fa             endbr64
   b31c4:       48 8b 15 e5 fc 14 00    mov    rdx,QWORD PTR [rip+0x14fce5]        # 202eb0 <_rtld_global_ro@GLIBC_PRIVATE>
   b31cb:       8b b2 b8 00 00 00       mov    esi,DWORD PTR [rdx+0xb8]
   b31d1:       8b 8a c4 01 00 00       mov    ecx,DWORD PTR [rdx+0x1c4]
   b31d7:       89 f0                   mov    eax,esi
   b31d9:       f7 d0                   not    eax
   b31db:       a9 20 01 00 00          test   eax,0x120
   b31e0:       74 2e                   je     b3210 <strcmp@@GLIBC_2.2.5+0x50>
   b31e2:       f6 82 9e 00 00 00 10    test   BYTE PTR [rdx+0x9e],0x10
   b31e9:       74 0c                   je     b31f7 <strcmp@@GLIBC_2.2.5+0x37>
   b31eb:       48 8d 05 4e 40 0f 00    lea    rax,[rip+0xf404e]        # 1a7240 <__nss_database_lookup@GLIBC_2.2.5+0x1f200>
   b31f2:       f6 c5 01                test   ch,0x1
   b31f5:       74 15                   je     b320c <strcmp@@GLIBC_2.2.5+0x4c>
   b31f7:       83 e1 08                and    ecx,0x8
   b31fa:       48 8d 05 3f dd 00 00    lea    rax,[rip+0xdd3f]        # c0f40 <memcpy@GLIBC_2.2.5+0x66d0>
   b3201:       48 8d 15 e8 c8 00 00    lea    rdx,[rip+0xc8e8]        # bfaf0 <memcpy@GLIBC_2.2.5+0x5280>
   b3208:       48 0f 44 c2             cmove  rax,rdx
   b320c:       c3                      ret
   b320d:       0f 1f 00                nop    DWORD PTR [rax]
   b3210:       f6 c5 02                test   ch,0x2
   b3213:       74 cd                   je     b31e2 <strcmp@@GLIBC_2.2.5+0x22>
   b3215:       85 f6                   test   esi,esi
   b3217:       78 1f                   js     b3238 <strcmp@@GLIBC_2.2.5+0x78>
   b3219:       81 e6 00 08 00 00       and    esi,0x800
   b321f:       48 8d 05 aa 08 0e 00    lea    rax,[rip+0xe08aa]        # 193ad0 <__nss_database_lookup@GLIBC_2.2.5+0xba90>
   b3226:       75 e4                   jne    b320c <strcmp@@GLIBC_2.2.5+0x4c>
   b3228:       48 8d 05 e1 7d 0d 00    lea    rax,[rip+0xd7de1]        # 18b010 <__nss_database_lookup@GLIBC_2.2.5+0x2fd0>
   b322f:       f6 c5 08                test   ch,0x8
   b3232:       75 ae                   jne    b31e2 <strcmp@@GLIBC_2.2.5+0x22>
   b3234:       c3                      ret
   b3235:       0f 1f 00                nop    DWORD PTR [rax]
   b3238:       48 8d 05 e1 7e 0e 00    lea    rax,[rip+0xe7ee1]        # 19b120 <__nss_database_lookup@GLIBC_2.2.5+0x130e0>
   b323f:       f7 c6 00 00 00 40       test   esi,0x40000000
   b3245:       74 d2                   je     b3219 <strcmp@@GLIBC_2.2.5+0x59>
   b3247:       c3                      ret
   b3248:       0f 1f 84 00 00 00 00    nop    DWORD PTR [rax+rax*1+0x0]


part of the optimized code: -> 

 193b52:       29 c8                   sub    eax,ecx
  193b54:       eb bb                   jmp    193b11 <__nss_database_lookup@GLIBC_2.2.5+0xbad1>
  193b56:       66 2e 0f 1f 84 00 00    cs nop WORD PTR [rax+rax*1+0x0]
  193b5d:       00 00 00 
  193b60:       c5 fe 6f 47 20          vmovdqu ymm0,YMMWORD PTR [rdi+0x20]
  193b65:       c5 fd 74 4e 20          vpcmpeqb ymm1,ymm0,YMMWORD PTR [rsi+0x20]
  193b6a:       c5 85 74 d0             vpcmpeqb ymm2,ymm15,ymm0
  193b6e:       c5 ed df c9             vpandn ymm1,ymm2,ymm1
  193b72:       c5 fd d7 c9             vpmovmskb ecx,ymm1
  193b76:       ff c1                   inc    ecx
  193b78:       75 a6                   jne    193b20 <__nss_database_lookup@GLIBC_2.2.5+0xbae0>
  193b7a:       c5 fe 6f 47 40          vmovdqu ymm0,YMMWORD PTR [rdi+0x40]
  193b7f:       c5 fd 74 4e 40          vpcmpeqb ymm1,ymm0,YMMWORD PTR [rsi+0x40]
  193b84:       c5 85 74 d0             vpcmpeqb ymm2,ymm15,ymm0
  193b88:       c5 ed df c9             vpandn ymm1,ymm2,ymm1
  193b8c:       c5 fd d7 c9             vpmovmskb ecx,ymm1
  193b90:       ff c1                   inc    ecx
  193b92:       75 9e                   jne    193b32 <__nss_database_lookup@GLIBC_2.2.5+0xbaf2>
  193b94:       c5 fe 6f 47 60          vmovdqu ymm0,YMMWORD PTR [rdi+0x60]
  193b99:       c5 fd 74 4e 60          vpcmpeqb ymm1,ymm0,YMMWORD PTR [rsi+0x60]
  193b9e:       c5 85 74 d0             vpcmpeqb ymm2,ymm15,ymm0
  193ba2:       c5 ed df c9             vpandn ymm1,ymm2,ymm1
  193ba6:       c5 fd d7 c9             vpmovmskb ecx,ymm1
  193baa:       ff c1                   inc    ecx
  193bac:       75 96                   jne    193b44 <__nss_database_lookup@GLIBC_2.2.5+0xbb04>
  193bae:       45 31 c0                xor    r8d,r8d
  193bb1:       48 29 fe                sub    rsi,rdi
  193bb4:       48 83 e7 80             and    rdi,0xffffffffffffff80
  193bb8:       48 01 fe                add    rsi,rdi
  193bbb:       b8 80 ff ff ff          mov    eax,0xffffff80
  193bc0:       29 f0                   sub    eax,esi
  193bc2:       25 ff 0f 00 00          and    eax,0xfff
  193bc7:       66 0f 1f 84 00 00 00    nop    WORD PTR [rax+rax*1+0x0]
  193bce:       00 00 
  193bd0:       48 83 ef 80             sub    rdi,0xffffffffffffff80
  193bd4:       48 83 ee 80             sub    rsi,0xffffffffffffff80
  193bd8:       83 c0 80                add    eax,0xffffff80
  193bdb:       0f 83 ef 00 00 00       jae    193cd0 <__nss_database_lookup@GLIBC_2.2.5+0xbc90>
  193be1:       c5 fd 6f 07             vmovdqa ymm0,YMMWORD PTR [rdi]
  193be5:       c5 fd 6f 57 20          vmovdqa ymm2,YMMWORD PTR [rdi+0x20]
  193bea:       c5 fd 6f 67 40          vmovdqa ymm4,YMMWORD PTR [rdi+0x40]
  193bef:       c5 fd 6f 77 60          vmovdqa ymm6,YMMWORD PTR [rdi+0x60]
  193bf4:       c5 fd 74 0e             vpcmpeqb ymm1,ymm0,YMMWORD PTR [rsi]
  193bf8:       c5 ed 74 5e 20          vpcmpeqb ymm3,ymm2,YMMWORD PTR [rsi+0x20]
  193bfd:       c5 dd 74 6e 40          vpcmpeqb ymm5,ymm4,YMMWORD PTR [rsi+0x40]
  193c02:       c5 cd 74 7e 60          vpcmpeqb ymm7,ymm6,YMMWORD PTR [rsi+0x60]
  193c07:       c5 f5 db c8             vpand  ymm1,ymm1,ymm0
  193c0b:       c5 e5 db da             vpand  ymm3,ymm3,ymm2
  193c0f:       c5 d5 db ec             vpand  ymm5,ymm5,ymm4
  193c13:       c5 c5 db fe             vpand  ymm7,ymm7,ymm6
  193c17:       c5 e5 da d9             vpminub ymm3,ymm3,ymm1
  193c1b:       c5 c5 da fd             vpminub ymm7,ymm7,ymm5
  193c1f:       c5 c5 da fb             vpminub ymm7,ymm7,ymm3
  193c23:       c5 85 74 ff             vpcmpeqb ymm7,ymm15,ymm7
  193c27:       c5 fd d7 d7             vpmovmskb edx,ymm7
  193c2b:       85 d2                   test   edx,edx
  193c2d:       74 a1                   je     193bd0 <__nss_database_lookup@GLIBC_2.2.5+0xbb90>
  193c2f:       c5 85 74 c9             vpcmpeqb ymm1,ymm15,ymm1
  193c33:       c5 fd d7 c9             vpmovmskb ecx,ymm1
  193c37:       85 c9                   test   ecx,ecx
  193c39:       75 35                   jne    193c70 <__nss_database_lookup@GLIBC_2.2.5+0xbc30>
  193c3b:       c5 85 74 db             vpcmpeqb ymm3,ymm15,ymm3
  193c3f:       c5 fd d7 cb             vpmovmskb ecx,ymm3
  193c43:       85 c9                   test   ecx,ecx
  193c45:       75 49                   jne    193c90 <__nss_database_lookup@GLIBC_2.2.5+0xbc50>
  193c47:       c5 85 74 ed             vpcmpeqb ymm5,ymm15,ymm5
  193c4b:       c5 fd d7 cd             vpmovmskb ecx,ymm5
  193c4f:       85 c9                   test   ecx,ecx
  193c51:       75 5d                   jne    193cb0 <__nss_database_lookup@GLIBC_2.2.5+0xbc70>
  193c53:       f3 0f bc d2             tzcnt  edx,edx
  193c57:       0f b6 44 17 60          movzx  eax,BYTE PTR [rdi+rdx*1+0x60]
  193c5c:       0f b6 4c 16 60          movzx  ecx,BYTE PTR [rsi+rdx*1+0x60]
  193c61:       29 c8                   sub    eax,ecx
  193c63:       44 31 c0                xor    eax,r8d
  193c66:       44 29 c0                sub    eax,r8d
  193c69:       e9 a3 fe ff ff          jmp    193b11 <__nss_database_lookup@GLIBC_2.2.5+0xbad1>
  193c6e:       66 90                   xchg   ax,ax
  193c70:       f3 0f bc c9             tzcnt  ecx,ecx
  193c74:       0f b6 04 0f             movzx  eax,BYTE PTR [rdi+rcx*1]
  193c78:       0f b6 0c 0e             movzx  ecx,BYTE PTR [rsi+rcx*1]
  193c7c:       29 c8                   sub    eax,ecx
  193c7e:       44                      rex.R
  193c7f:       31                      .byte 0x31