import{s as Bt,d as Ee,r as Dt,n as M}from"../chunks/scheduler.CktBsrYA.js";import{S as qt,i as wt,e as o,s as j,c as r,a as m,n as f,k as C,d,b as a,o as p,f as pe,l as t,p as V,q as Y,r as Ht,u as Lt,g as Me,v as Vt,t as Ue,w as kt,h as ze,j as Fe,x as Mt,y as Ie,z as Nt,m as Ut,A as zt,B as Ft,C as Rt,D as Qt}from"../chunks/index.0K5uXL7a.js";import{e as St,h as gt,u as Wt,L as Gt}from"../chunks/List.BSQP8aP-.js";function Jt(n,e,u){const s=n.slice();return s[20]=e[u],s}function Kt(n,e,u){const s=n.slice();return s[8]=e[u],s}function Xt(n){let e,u=(n[8]||"Any")+"",s,i;return{c(){e=o("option"),s=ze(u),i=j(),this.h()},l(l){e=r(l,"OPTION",{});var _=m(e);s=Fe(_,u),i=C(_),_.forEach(d),this.h()},h(){e.__value=n[8],p(e,e.__value)},m(l,_){pe(l,e,_),t(e,s),t(e,i)},p:M,d(l){l&&d(e)}}}function Yt(n){let e,u=(n[20]||"Any")+"",s,i;return{c(){e=o("option"),s=ze(u),i=j(),this.h()},l(l){e=r(l,"OPTION",{});var _=m(e);s=Fe(_,u),i=C(_),_.forEach(d),this.h()},h(){e.__value=n[20],p(e,e.__value)},m(l,_){pe(l,e,_),t(e,s),t(e,i)},p:M,d(l){l&&d(e)}}}function Zt(n){let e,u="Pull a Query and the records will be displayed here";return{c(){e=o("span"),e.textContent=u},l(s){e=r(s,"SPAN",{"data-svelte-h":!0}),f(e)!=="svelte-f87e4e"&&(e.textContent=u)},m(s,i){pe(s,e,i)},p:M,i:M,o:M,d(s){s&&d(e)}}}function $t(n){let e,u,s,i={ctx:n,current:null,token:null,hasCatch:!0,pending:ll,then:tl,catch:el,value:18,error:19,blocks:[,,,]};return gt(u=n[7],i),{c(){e=Nt(),i.block.c()},l(l){e=Nt(),i.block.l(l)},m(l,_){pe(l,e,_),i.block.m(l,i.anchor=_),i.mount=()=>e.parentNode,i.anchor=e,s=!0},p(l,_){n=l,i.ctx=n,_&128&&u!==(u=n[7])&&gt(u,i)||Wt(i,n,_)},i(l){s||(Ue(i.block),s=!0)},o(l){for(let _=0;_<3;_+=1){const E=i.blocks[_];Me(E)}s=!1},d(l){l&&d(e),i.block.d(l),i.token=null,i=null}}}function el(n){let e,u,s=n[19]+"",i;return{c(){e=o("span"),u=ze("Failed: "),i=ze(s)},l(l){e=r(l,"SPAN",{});var _=m(e);u=Fe(_,"Failed: "),i=Fe(_,s),_.forEach(d)},m(l,_){pe(l,e,_),t(e,u),t(e,i)},p(l,_){_&128&&s!==(s=l[19]+"")&&Ut(i,s)},i:M,o:M,d(l){l&&d(e)}}}function tl(n){let e,u;return e=new Gt({props:{records:n[18]}}),{c(){zt(e.$$.fragment)},l(s){Ft(e.$$.fragment,s)},m(s,i){Rt(e,s,i),u=!0},p(s,i){const l={};i&128&&(l.records=s[18]),e.$set(l)},i(s){u||(Ue(e.$$.fragment,s),u=!0)},o(s){Me(e.$$.fragment,s),u=!1},d(s){Qt(e,s)}}}function ll(n){let e,u="Loading....";return{c(){e=o("span"),e.textContent=u},l(s){e=r(s,"SPAN",{"data-svelte-h":!0}),f(e)!=="svelte-16ntdv6"&&(e.textContent=u)},m(s,i){pe(s,e,i)},p:M,i:M,o:M,d(s){s&&d(e)}}}function nl(n){let e,u,s="JOSAA Counselling Query",i,l,_,E,fe=`Name of institute
                <span class="tooltip svelte-1x5j5jy"><span class="svelte-1x5j5jy">?</span> <span class="svelte-1x5j5jy">Don&#39;t use abbreviations like NIT, IIT. Use words even
                        just one like National to describe NIT.</span></span>`,Z,y,he,B,D,Oe=`Filter courses
                <span class="tooltip svelte-1x5j5jy"><span class="svelte-1x5j5jy">?</span> <span class="svelte-1x5j5jy">Don&#39;t use abbreviations like BE, CSE. Use words even
                        just one like Computer to describe CSE course.</span></span>`,xe,I,ye,g,q,Te=`Select Quota Type

                <span class="tooltip svelte-1x5j5jy"><span class="svelte-1x5j5jy">?</span> <span class="svelte-1x5j5jy">HS, OS, AI <small class="svelte-1x5j5jy">(All India)</small>, AP
                        <small class="svelte-1x5j5jy">(NIT Warangal)</small>, LA
                        <small class="svelte-1x5j5jy">(NIT Srinagar)</small>, JK
                        <small class="svelte-1x5j5jy">(NIT Srinagar)</small>, GO
                        <small class="svelte-1x5j5jy">(NIT Goa)</small></span></span>`,ke,b,T,Ne="Any ",k,Se="HS (Home State) ",U,nt="OS (Other State) ",z,st="AI (All India - for IITs and IIITs)",Re,$,ee,at="Select Seat Type",Qe,x,w,ot="Any ",F,rt="OPEN ",R,ut="EWS ",Q,it="OBC-NCL ",W,ct="SC ",G,_t="ST ",J,vt="PwD (OPEN, EWS, OBC-NCL, SC, ST)",We,te,le,ft="Select Gender",Ge,O,H,dt="Any ",K,pt="Neutral ",X,ht="Female Only (+Supernumerary)",Je,ne,se,jt="Opening Rank",Ke,je,L,Xe,ae,oe,Ct="Year",Ye,P,Ze,re,ue,mt="Round",$e,A,et,ie,xt="Query",tt,N,S,ge,lt,yt,Pt=St([null,2016,2017,2018,2019,2020,2021,2022,2023]),Ce=[];for(let c=0;c<9;c+=1)Ce[c]=Xt(Kt(n,Pt,c));let At=St([null,1,2,3,4,5,6]),me=[];for(let c=0;c<7;c+=1)me[c]=Yt(Jt(n,At,c));const bt=[$t,Zt],ce=[];function Et(c,h){return c[7]?0:1}return N=Et(n),S=ce[N]=bt[N](n),{c(){e=o("div"),u=o("h2"),u.textContent=s,i=j(),l=o("form"),_=o("div"),E=o("label"),E.innerHTML=fe,Z=j(),y=o("input"),he=j(),B=o("div"),D=o("label"),D.innerHTML=Oe,xe=j(),I=o("input"),ye=j(),g=o("div"),q=o("label"),q.innerHTML=Te,ke=j(),b=o("select"),T=o("option"),T.textContent=Ne,k=o("option"),k.textContent=Se,U=o("option"),U.textContent=nt,z=o("option"),z.textContent=st,Re=j(),$=o("div"),ee=o("label"),ee.textContent=at,Qe=j(),x=o("select"),w=o("option"),w.textContent=ot,F=o("option"),F.textContent=rt,R=o("option"),R.textContent=ut,Q=o("option"),Q.textContent=it,W=o("option"),W.textContent=ct,G=o("option"),G.textContent=_t,J=o("option"),J.textContent=vt,We=j(),te=o("div"),le=o("label"),le.textContent=ft,Ge=j(),O=o("select"),H=o("option"),H.textContent=dt,K=o("option"),K.textContent=pt,X=o("option"),X.textContent=ht,Je=j(),ne=o("div"),se=o("label"),se.textContent=jt,Ke=j(),je=o("div"),L=o("input"),Xe=j(),ae=o("div"),oe=o("label"),oe.textContent=Ct,Ye=j(),P=o("select");for(let c=0;c<9;c+=1)Ce[c].c();Ze=j(),re=o("div"),ue=o("label"),ue.textContent=mt,$e=j(),A=o("select");for(let c=0;c<7;c+=1)me[c].c();et=j(),ie=o("button"),ie.textContent=xt,tt=j(),S.c(),this.h()},l(c){e=r(c,"DIV",{class:!0});var h=m(e);u=r(h,"H2",{class:!0,"data-svelte-h":!0}),f(u)!=="svelte-14yo3o9"&&(u.textContent=s),i=C(h),l=r(h,"FORM",{class:!0});var v=m(l);_=r(v,"DIV",{class:!0});var Le=m(_);E=r(Le,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(E)!=="svelte-1l3euz4"&&(E.innerHTML=fe),Z=C(Le),y=r(Le,"INPUT",{name:!0,placeholder:!0,type:!0,class:!0}),Le.forEach(d),he=C(v),B=r(v,"DIV",{class:!0});var Pe=m(B);D=r(Pe,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(D)!=="svelte-d83jde"&&(D.innerHTML=Oe),xe=C(Pe),I=r(Pe,"INPUT",{name:!0,placeholder:!0,type:!0,class:!0}),Pe.forEach(d),ye=C(v),g=r(v,"DIV",{id:!0,class:!0});var Ae=m(g);q=r(Ae,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(q)!=="svelte-1eya8oi"&&(q.innerHTML=Te),ke=C(Ae),b=r(Ae,"SELECT",{name:!0,class:!0});var be=m(b);T=r(be,"OPTION",{"data-svelte-h":!0}),f(T)!=="svelte-11soqor"&&(T.textContent=Ne),k=r(be,"OPTION",{"data-svelte-h":!0}),f(k)!=="svelte-1g8i27x"&&(k.textContent=Se),U=r(be,"OPTION",{"data-svelte-h":!0}),f(U)!=="svelte-17wq9e4"&&(U.textContent=nt),z=r(be,"OPTION",{"data-svelte-h":!0}),f(z)!=="svelte-1vis8cj"&&(z.textContent=st),be.forEach(d),Ae.forEach(d),Re=C(v),$=r(v,"DIV",{class:!0});var Be=m($);ee=r(Be,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(ee)!=="svelte-pj6eei"&&(ee.textContent=at),Qe=C(Be),x=r(Be,"SELECT",{name:!0,class:!0});var _e=m(x);w=r(_e,"OPTION",{"data-svelte-h":!0}),f(w)!=="svelte-11soqor"&&(w.textContent=ot),F=r(_e,"OPTION",{"data-svelte-h":!0}),f(F)!=="svelte-13r574e"&&(F.textContent=rt),R=r(_e,"OPTION",{"data-svelte-h":!0}),f(R)!=="svelte-1iep8sm"&&(R.textContent=ut),Q=r(_e,"OPTION",{"data-svelte-h":!0}),f(Q)!=="svelte-1hj34ty"&&(Q.textContent=it),W=r(_e,"OPTION",{"data-svelte-h":!0}),f(W)!=="svelte-cgt2z2"&&(W.textContent=ct),G=r(_e,"OPTION",{"data-svelte-h":!0}),f(G)!=="svelte-3c6kyo"&&(G.textContent=_t),J=r(_e,"OPTION",{"data-svelte-h":!0}),f(J)!=="svelte-ikne3d"&&(J.textContent=vt),_e.forEach(d),Be.forEach(d),We=C(v),te=r(v,"DIV",{class:!0});var De=m(te);le=r(De,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(le)!=="svelte-1xikm4u"&&(le.textContent=ft),Ge=C(De),O=r(De,"SELECT",{name:!0,class:!0});var qe=m(O);H=r(qe,"OPTION",{"data-svelte-h":!0}),f(H)!=="svelte-11soqor"&&(H.textContent=dt),K=r(qe,"OPTION",{"data-svelte-h":!0}),f(K)!=="svelte-8ppvv6"&&(K.textContent=pt),X=r(qe,"OPTION",{"data-svelte-h":!0}),f(X)!=="svelte-1q1duci"&&(X.textContent=ht),qe.forEach(d),De.forEach(d),Je=C(v),ne=r(v,"DIV",{class:!0});var we=m(ne);se=r(we,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(se)!=="svelte-4zxxuu"&&(se.textContent=jt),Ke=C(we),je=r(we,"DIV",{class:!0});var It=m(je);L=r(It,"INPUT",{name:!0,placeholder:!0,type:!0,class:!0}),It.forEach(d),we.forEach(d),Xe=C(v),ae=r(v,"DIV",{class:!0});var He=m(ae);oe=r(He,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(oe)!=="svelte-11k8a8u"&&(oe.textContent=Ct),Ye=C(He),P=r(He,"SELECT",{name:!0,class:!0});var Ot=m(P);for(let de=0;de<9;de+=1)Ce[de].l(Ot);Ot.forEach(d),He.forEach(d),Ze=C(v),re=r(v,"DIV",{class:!0});var Ve=m(re);ue=r(Ve,"LABEL",{for:!0,class:!0,"data-svelte-h":!0}),f(ue)!=="svelte-1dhz85m"&&(ue.textContent=mt),$e=C(Ve),A=r(Ve,"SELECT",{name:!0,class:!0});var Tt=m(A);for(let de=0;de<7;de+=1)me[de].l(Tt);Tt.forEach(d),Ve.forEach(d),et=C(v),ie=r(v,"BUTTON",{type:!0,class:!0,"data-svelte-h":!0}),f(ie)!=="svelte-1z7hgf"&&(ie.textContent=xt),v.forEach(d),tt=C(h),S.l(h),h.forEach(d),this.h()},h(){a(u,"class","svelte-1x5j5jy"),a(E,"for","institute"),a(E,"class","svelte-1x5j5jy"),a(y,"name","institute"),a(y,"placeholder","Eg: Birla, Indian Institute"),a(y,"type","text"),a(y,"class","svelte-1x5j5jy"),a(_,"class","svelte-1x5j5jy"),a(D,"for","course"),a(D,"class","svelte-1x5j5jy"),a(I,"name","course"),a(I,"placeholder","Eg: Mechanical, Civil"),a(I,"type","text"),a(I,"class","svelte-1x5j5jy"),a(B,"class","svelte-1x5j5jy"),a(q,"for","quota"),a(q,"class","svelte-1x5j5jy"),T.__value="",p(T,T.__value),T.selected=!0,k.__value="HS",p(k,k.__value),U.__value="OS",p(U,U.__value),z.__value="AI",p(z,z.__value),a(b,"name","quota"),a(b,"class","svelte-1x5j5jy"),n[2]===void 0&&Ee(()=>n[12].call(b)),a(g,"id","quota"),a(g,"class","svelte-1x5j5jy"),a(ee,"for","seat"),a(ee,"class","svelte-1x5j5jy"),w.__value="",p(w,w.__value),w.selected=!0,F.__value="OPEN",p(F,F.__value),R.__value="EWS",p(R,R.__value),Q.__value="OBC-NCL",p(Q,Q.__value),W.__value="SC",p(W,W.__value),G.__value="ST",p(G,G.__value),J.__value="PwD",p(J,J.__value),a(x,"name","seat"),a(x,"class","svelte-1x5j5jy"),n[3]===void 0&&Ee(()=>n[13].call(x)),a($,"class","svelte-1x5j5jy"),a(le,"for","gender"),a(le,"class","svelte-1x5j5jy"),H.__value="",p(H,H.__value),H.selected=!0,K.__value="neutral",p(K,K.__value),X.__value="female",p(X,X.__value),a(O,"name","gender"),a(O,"class","svelte-1x5j5jy"),n[4]===void 0&&Ee(()=>n[14].call(O)),a(te,"class","svelte-1x5j5jy"),a(se,"for","rank"),a(se,"class","svelte-1x5j5jy"),a(L,"name","rank"),a(L,"placeholder","Eg. 14000"),a(L,"type","number"),a(L,"class","svelte-1x5j5jy"),a(je,"class","svelte-1x5j5jy"),a(ne,"class","svelte-1x5j5jy"),a(oe,"for","year"),a(oe,"class","svelte-1x5j5jy"),a(P,"name","year"),a(P,"class","svelte-1x5j5jy"),n[8]===void 0&&Ee(()=>n[16].call(P)),a(ae,"class","svelte-1x5j5jy"),a(ue,"for","round"),a(ue,"class","svelte-1x5j5jy"),a(A,"name","round"),a(A,"class","svelte-1x5j5jy"),n[6]===void 0&&Ee(()=>n[17].call(A)),a(re,"class","svelte-1x5j5jy"),a(ie,"type","submit"),a(ie,"class","svelte-1x5j5jy"),a(l,"class","svelte-1x5j5jy"),a(e,"class","container svelte-1x5j5jy")},m(c,h){pe(c,e,h),t(e,u),t(e,i),t(e,l),t(l,_),t(_,E),t(_,Z),t(_,y),p(y,n[0]),t(l,he),t(l,B),t(B,D),t(B,xe),t(B,I),p(I,n[1]),t(l,ye),t(l,g),t(g,q),t(g,ke),t(g,b),t(b,T),t(b,k),t(b,U),t(b,z),V(b,n[2],!0),t(l,Re),t(l,$),t($,ee),t($,Qe),t($,x),t(x,w),t(x,F),t(x,R),t(x,Q),t(x,W),t(x,G),t(x,J),V(x,n[3],!0),t(l,We),t(l,te),t(te,le),t(te,Ge),t(te,O),t(O,H),t(O,K),t(O,X),V(O,n[4],!0),t(l,Je),t(l,ne),t(ne,se),t(ne,Ke),t(ne,je),t(je,L),p(L,n[5]),t(l,Xe),t(l,ae),t(ae,oe),t(ae,Ye),t(ae,P);for(let v=0;v<9;v+=1)Ce[v]&&Ce[v].m(P,null);V(P,n[8],!0),t(l,Ze),t(l,re),t(re,ue),t(re,$e),t(re,A);for(let v=0;v<7;v+=1)me[v]&&me[v].m(A,null);V(A,n[6],!0),t(l,et),t(l,ie),t(e,tt),ce[N].m(e,null),ge=!0,lt||(yt=[Y(y,"input",n[10]),Y(I,"input",n[11]),Y(b,"change",n[12]),Y(x,"change",n[13]),Y(O,"change",n[14]),Y(L,"input",n[15]),Y(P,"change",n[16]),Y(A,"change",n[17]),Y(l,"submit",Ht(n[9]))],lt=!0)},p(c,[h]){h&1&&y.value!==c[0]&&p(y,c[0]),h&2&&I.value!==c[1]&&p(I,c[1]),h&4&&V(b,c[2]),h&8&&V(x,c[3]),h&16&&V(O,c[4]),h&32&&Lt(L.value)!==c[5]&&p(L,c[5]),h&256&&V(P,c[8]),h&64&&V(A,c[6]);let v=N;N=Et(c),N===v?ce[N].p(c,h):(Mt(),Me(ce[v],1,1,()=>{ce[v]=null}),Vt(),S=ce[N],S?S.p(c,h):(S=ce[N]=bt[N](c),S.c()),Ue(S,1),S.m(e,null))},i(c){ge||(Ue(S),ge=!0)},o(c){Me(S),ge=!1},d(c){c&&d(e),kt(Ce,c),kt(me,c),ce[N].d(),lt=!1,Dt(yt)}}}const sl="/josaa?";function ve(n,e){return n&&e?`${n}=${e}&`:""}function al(n,e,u){let s,i,l,_,E,fe,Z,y,he;function B(ke){let b=sl+ve("name",s)+ve("course",i)+ve("quote",l)+ve("seat",_)+ve("gender",E)+ve("rank",fe)+ve("year",Z)+ve("round",y);u(7,he=new Promise(async(T,Ne)=>{try{let Se=await(await fetch(b)).json();T(Se)}catch(k){Ne(k)}}))}function D(){s=this.value,u(0,s)}function Oe(){i=this.value,u(1,i)}function xe(){l=Ie(this),u(2,l)}function I(){_=Ie(this),u(3,_)}function ye(){E=Ie(this),u(4,E)}function g(){fe=Lt(this.value),u(5,fe)}function q(){Z=Ie(this),u(8,Z)}function Te(){y=Ie(this),u(6,y)}return[s,i,l,_,E,fe,y,he,Z,B,D,Oe,xe,I,ye,g,q,Te]}class il extends qt{constructor(e){super(),wt(this,e,al,nl,Bt,{})}}export{il as component};