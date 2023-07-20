var e="top",t="bottom",n="right",r="left",o="auto",i=[e,t,n,r],a="start",s="end",f="clippingParents",c="viewport",p="popper",u="reference",l=i.reduce((function(e,t){return e.concat([t+"-start",t+"-"+s])}),[]),d=[].concat(i,["auto"]).reduce((function(e,t){return e.concat([t,t+"-start",t+"-"+s])}),[]),h="beforeRead",m="read",v="afterRead",g="beforeMain",y="main",b="afterMain",w="beforeWrite",x="write",O="afterWrite",j=["beforeRead","read","afterRead","beforeMain","main","afterMain","beforeWrite","write","afterWrite"];function E(e){return e?(e.nodeName||"").toLowerCase():null}function D(e){if(null==e)return window;if("[object Window]"!==e.toString()){var t=e.ownerDocument;return t&&t.defaultView||window}return e}function A(e){return e instanceof D(e).Element||e instanceof Element}function L(e){return e instanceof D(e).HTMLElement||e instanceof HTMLElement}function M(e){return"undefined"!=typeof ShadowRoot&&(e instanceof D(e).ShadowRoot||e instanceof ShadowRoot)}var P={name:"applyStyles",enabled:!0,phase:"write",fn:function(e){var t=e.state;Object.keys(t.elements).forEach((function(e){var n=t.styles[e]||{},r=t.attributes[e]||{},o=t.elements[e];L(o)&&E(o)&&(Object.assign(o.style,n),Object.keys(r).forEach((function(e){var t=r[e];!1===t?o.removeAttribute(e):o.setAttribute(e,!0===t?"":t)})))}))},effect:function(e){var t=e.state,n={popper:{position:t.options.strategy,left:"0",top:"0",margin:"0"},arrow:{position:"absolute"},reference:{}};return Object.assign(t.elements.popper.style,n.popper),t.styles=n,t.elements.arrow&&Object.assign(t.elements.arrow.style,n.arrow),function(){Object.keys(t.elements).forEach((function(e){var r=t.elements[e],o=t.attributes[e]||{},i=Object.keys(t.styles.hasOwnProperty(e)?t.styles[e]:n[e]).reduce((function(e,t){return e[t]="",e}),{});L(r)&&E(r)&&(Object.assign(r.style,i),Object.keys(o).forEach((function(e){r.removeAttribute(e)})))}))}},requires:["computeStyles"]};function W(e){return e.split("-")[0]}var k=Math.max,B=Math.min,H=Math.round;function R(){var e=navigator.userAgentData;return null!=e&&e.brands&&Array.isArray(e.brands)?e.brands.map((function(e){return e.brand+"/"+e.version})).join(" "):navigator.userAgent}function T(){return!/^((?!chrome|android).)*safari/i.test(R())}function S(e,t,n){void 0===t&&(t=!1),void 0===n&&(n=!1);var r=e.getBoundingClientRect(),o=1,i=1;t&&L(e)&&(o=e.offsetWidth>0&&H(r.width)/e.offsetWidth||1,i=e.offsetHeight>0&&H(r.height)/e.offsetHeight||1);var a=(A(e)?D(e):window).visualViewport,s=!T()&&n,f=(r.left+(s&&a?a.offsetLeft:0))/o,c=(r.top+(s&&a?a.offsetTop:0))/i,p=r.width/o,u=r.height/i;return{width:p,height:u,top:c,right:f+p,bottom:c+u,left:f,x:f,y:c}}function V(e){var t=S(e),n=e.offsetWidth,r=e.offsetHeight;return Math.abs(t.width-n)<=1&&(n=t.width),Math.abs(t.height-r)<=1&&(r=t.height),{x:e.offsetLeft,y:e.offsetTop,width:n,height:r}}function q(e,t){var n=t.getRootNode&&t.getRootNode();if(e.contains(t))return!0;if(n&&M(n)){var r=t;do{if(r&&e.isSameNode(r))return!0;r=r.parentNode||r.host}while(r)}return!1}function C(e){return D(e).getComputedStyle(e)}function N(e){return["table","td","th"].indexOf(E(e))>=0}function I(e){return((A(e)?e.ownerDocument:e.document)||window.document).documentElement}function F(e){return"html"===E(e)?e:e.assignedSlot||e.parentNode||(M(e)?e.host:null)||I(e)}function U(e){return L(e)&&"fixed"!==C(e).position?e.offsetParent:null}function z(e){for(var t=D(e),n=U(e);n&&N(n)&&"static"===C(n).position;)n=U(n);return n&&("html"===E(n)||"body"===E(n)&&"static"===C(n).position)?t:n||function(e){var t=/firefox/i.test(R());if(/Trident/i.test(R())&&L(e)&&"fixed"===C(e).position)return null;var n=F(e);for(M(n)&&(n=n.host);L(n)&&["html","body"].indexOf(E(n))<0;){var r=C(n);if("none"!==r.transform||"none"!==r.perspective||"paint"===r.contain||-1!==["transform","perspective"].indexOf(r.willChange)||t&&"filter"===r.willChange||t&&r.filter&&"none"!==r.filter)return n;n=n.parentNode}return null}(e)||t}function _(e){return["top","bottom"].indexOf(e)>=0?"x":"y"}function X(e,t,n){return k(e,B(t,n))}function Y(e){return Object.assign({},{top:0,right:0,bottom:0,left:0},e)}function G(e,t){return t.reduce((function(t,n){return t[n]=e,t}),{})}var J={name:"arrow",enabled:!0,phase:"main",fn:function(o){var a,s=o.state,f=o.name,c=o.options,p=s.elements.arrow,u=s.modifiersData.popperOffsets,l=W(s.placement),d=_(l),h=[r,n].indexOf(l)>=0?"height":"width";if(p&&u){var m=function(e,t){return Y("number"!=typeof(e="function"==typeof e?e(Object.assign({},t.rects,{placement:t.placement})):e)?e:G(e,i))}(c.padding,s),v=V(p),g="y"===d?e:r,y="y"===d?t:n,b=s.rects.reference[h]+s.rects.reference[d]-u[d]-s.rects.popper[h],w=u[d]-s.rects.reference[d],x=z(p),O=x?"y"===d?x.clientHeight||0:x.clientWidth||0:0,j=b/2-w/2,E=m[g],D=O-v[h]-m[y],A=O/2-v[h]/2+j,L=X(E,A,D),M=d;s.modifiersData[f]=((a={})[M]=L,a.centerOffset=L-A,a)}},effect:function(e){var t=e.state,n=e.options.element,r=void 0===n?"[data-popper-arrow]":n;null!=r&&("string"!=typeof r||(r=t.elements.popper.querySelector(r)))&&q(t.elements.popper,r)&&(t.elements.arrow=r)},requires:["popperOffsets"],requiresIfExists:["preventOverflow"]};function K(e){return e.split("-")[1]}var Q={top:"auto",right:"auto",bottom:"auto",left:"auto"};function Z(o){var i,a=o.popper,f=o.popperRect,c=o.placement,p=o.variation,u=o.offsets,l=o.position,d=o.gpuAcceleration,h=o.adaptive,m=o.roundOffsets,v=o.isFixed,g=u.x,y=void 0===g?0:g,b=u.y,w=void 0===b?0:b,x="function"==typeof m?m({x:y,y:w}):{x:y,y:w};y=x.x,w=x.y;var O=u.hasOwnProperty("x"),j=u.hasOwnProperty("y"),E=r,A=e,L=window;if(h){var M=z(a),P="clientHeight",W="clientWidth";if(M===D(a)&&"static"!==C(M=I(a)).position&&"absolute"===l&&(P="scrollHeight",W="scrollWidth"),M=M,c===e||(c===r||c===n)&&p===s)A=t,w-=(v&&M===L&&L.visualViewport?L.visualViewport.height:M[P])-f.height,w*=d?1:-1;if(c===r||(c===e||c===t)&&p===s)E=n,y-=(v&&M===L&&L.visualViewport?L.visualViewport.width:M[W])-f.width,y*=d?1:-1}var k,B=Object.assign({position:l},h&&Q),R=!0===m?function(e,t){var n=e.x,r=e.y,o=t.devicePixelRatio||1;return{x:H(n*o)/o||0,y:H(r*o)/o||0}}({x:y,y:w},D(a)):{x:y,y:w};return y=R.x,w=R.y,d?Object.assign({},B,((k={})[A]=j?"0":"",k[E]=O?"0":"",k.transform=(L.devicePixelRatio||1)<=1?"translate("+y+"px, "+w+"px)":"translate3d("+y+"px, "+w+"px, 0)",k)):Object.assign({},B,((i={})[A]=j?w+"px":"",i[E]=O?y+"px":"",i.transform="",i))}var $={name:"computeStyles",enabled:!0,phase:"beforeWrite",fn:function(e){var t=e.state,n=e.options,r=n.gpuAcceleration,o=void 0===r||r,i=n.adaptive,a=void 0===i||i,s=n.roundOffsets,f=void 0===s||s,c={placement:W(t.placement),variation:K(t.placement),popper:t.elements.popper,popperRect:t.rects.popper,gpuAcceleration:o,isFixed:"fixed"===t.options.strategy};null!=t.modifiersData.popperOffsets&&(t.styles.popper=Object.assign({},t.styles.popper,Z(Object.assign({},c,{offsets:t.modifiersData.popperOffsets,position:t.options.strategy,adaptive:a,roundOffsets:f})))),null!=t.modifiersData.arrow&&(t.styles.arrow=Object.assign({},t.styles.arrow,Z(Object.assign({},c,{offsets:t.modifiersData.arrow,position:"absolute",adaptive:!1,roundOffsets:f})))),t.attributes.popper=Object.assign({},t.attributes.popper,{"data-popper-placement":t.placement})},data:{}},ee={passive:!0};var te={name:"eventListeners",enabled:!0,phase:"write",fn:function(){},effect:function(e){var t=e.state,n=e.instance,r=e.options,o=r.scroll,i=void 0===o||o,a=r.resize,s=void 0===a||a,f=D(t.elements.popper),c=[].concat(t.scrollParents.reference,t.scrollParents.popper);return i&&c.forEach((function(e){e.addEventListener("scroll",n.update,ee)})),s&&f.addEventListener("resize",n.update,ee),function(){i&&c.forEach((function(e){e.removeEventListener("scroll",n.update,ee)})),s&&f.removeEventListener("resize",n.update,ee)}},data:{}},ne={left:"right",right:"left",bottom:"top",top:"bottom"};function re(e){return e.replace(/left|right|bottom|top/g,(function(e){return ne[e]}))}var oe={start:"end",end:"start"};function ie(e){return e.replace(/start|end/g,(function(e){return oe[e]}))}function ae(e){var t=D(e);return{scrollLeft:t.pageXOffset,scrollTop:t.pageYOffset}}function se(e){return S(I(e)).left+ae(e).scrollLeft}function fe(e){var t=C(e),n=t.overflow,r=t.overflowX,o=t.overflowY;return/auto|scroll|overlay|hidden/.test(n+o+r)}function ce(e){return["html","body","#document"].indexOf(E(e))>=0?e.ownerDocument.body:L(e)&&fe(e)?e:ce(F(e))}function pe(e,t){var n;void 0===t&&(t=[]);var r=ce(e),o=r===(null==(n=e.ownerDocument)?void 0:n.body),i=D(r),a=o?[i].concat(i.visualViewport||[],fe(r)?r:[]):r,s=t.concat(a);return o?s:s.concat(pe(F(a)))}function ue(e){return Object.assign({},e,{left:e.x,top:e.y,right:e.x+e.width,bottom:e.y+e.height})}function le(e,t,n){return"viewport"===t?ue(function(e,t){var n=D(e),r=I(e),o=n.visualViewport,i=r.clientWidth,a=r.clientHeight,s=0,f=0;if(o){i=o.width,a=o.height;var c=T();(c||!c&&"fixed"===t)&&(s=o.offsetLeft,f=o.offsetTop)}return{width:i,height:a,x:s+se(e),y:f}}(e,n)):A(t)?function(e,t){var n=S(e,!1,"fixed"===t);return n.top=n.top+e.clientTop,n.left=n.left+e.clientLeft,n.bottom=n.top+e.clientHeight,n.right=n.left+e.clientWidth,n.width=e.clientWidth,n.height=e.clientHeight,n.x=n.left,n.y=n.top,n}(t,n):ue(function(e){var t,n=I(e),r=ae(e),o=null==(t=e.ownerDocument)?void 0:t.body,i=k(n.scrollWidth,n.clientWidth,o?o.scrollWidth:0,o?o.clientWidth:0),a=k(n.scrollHeight,n.clientHeight,o?o.scrollHeight:0,o?o.clientHeight:0),s=-r.scrollLeft+se(e),f=-r.scrollTop;return"rtl"===C(o||n).direction&&(s+=k(n.clientWidth,o?o.clientWidth:0)-i),{width:i,height:a,x:s,y:f}}(I(e)))}function de(e,t,n,r){var o="clippingParents"===t?function(e){var t=pe(F(e)),n=["absolute","fixed"].indexOf(C(e).position)>=0&&L(e)?z(e):e;return A(n)?t.filter((function(e){return A(e)&&q(e,n)&&"body"!==E(e)})):[]}(e):[].concat(t),i=[].concat(o,[n]),a=i[0],s=i.reduce((function(t,n){var o=le(e,n,r);return t.top=k(o.top,t.top),t.right=B(o.right,t.right),t.bottom=B(o.bottom,t.bottom),t.left=k(o.left,t.left),t}),le(e,a,r));return s.width=s.right-s.left,s.height=s.bottom-s.top,s.x=s.left,s.y=s.top,s}function he(o){var i,a=o.reference,f=o.element,c=o.placement,p=c?W(c):null,u=c?K(c):null,l=a.x+a.width/2-f.width/2,d=a.y+a.height/2-f.height/2;switch(p){case e:i={x:l,y:a.y-f.height};break;case t:i={x:l,y:a.y+a.height};break;case n:i={x:a.x+a.width,y:d};break;case r:i={x:a.x-f.width,y:d};break;default:i={x:a.x,y:a.y}}var h=p?_(p):null;if(null!=h){var m="y"===h?"height":"width";switch(u){case"start":i[h]=i[h]-(a[m]/2-f[m]/2);break;case s:i[h]=i[h]+(a[m]/2-f[m]/2)}}return i}function me(r,o){void 0===o&&(o={});var a=o,s=a.placement,f=void 0===s?r.placement:s,c=a.strategy,p=void 0===c?r.strategy:c,u=a.boundary,l=void 0===u?"clippingParents":u,d=a.rootBoundary,h=void 0===d?"viewport":d,m=a.elementContext,v=void 0===m?"popper":m,g=a.altBoundary,y=void 0!==g&&g,b=a.padding,w=void 0===b?0:b,x=Y("number"!=typeof w?w:G(w,i)),O="popper"===v?"reference":"popper",j=r.rects.popper,E=r.elements[y?O:v],D=de(A(E)?E:E.contextElement||I(r.elements.popper),l,h,p),L=S(r.elements.reference),M=he({reference:L,element:j,strategy:"absolute",placement:f}),P=ue(Object.assign({},j,M)),W="popper"===v?P:L,k={top:D.top-W.top+x.top,bottom:W.bottom-D.bottom+x.bottom,left:D.left-W.left+x.left,right:W.right-D.right+x.right},B=r.modifiersData.offset;if("popper"===v&&B){var H=B[f];Object.keys(k).forEach((function(r){var o=[n,t].indexOf(r)>=0?1:-1,i=[e,t].indexOf(r)>=0?"y":"x";k[r]+=H[i]*o}))}return k}function ve(e,t){void 0===t&&(t={});var n=t,r=n.placement,o=n.boundary,a=n.rootBoundary,s=n.padding,f=n.flipVariations,c=n.allowedAutoPlacements,p=void 0===c?d:c,u=K(r),h=u?f?l:l.filter((function(e){return K(e)===u})):i,m=h.filter((function(e){return p.indexOf(e)>=0}));0===m.length&&(m=h);var v=m.reduce((function(t,n){return t[n]=me(e,{placement:n,boundary:o,rootBoundary:a,padding:s})[W(n)],t}),{});return Object.keys(v).sort((function(e,t){return v[e]-v[t]}))}var ge={name:"flip",enabled:!0,phase:"main",fn:function(o){var i=o.state,a=o.options,s=o.name;if(!i.modifiersData[s]._skip){for(var f=a.mainAxis,c=void 0===f||f,p=a.altAxis,u=void 0===p||p,l=a.fallbackPlacements,d=a.padding,h=a.boundary,m=a.rootBoundary,v=a.altBoundary,g=a.flipVariations,y=void 0===g||g,b=a.allowedAutoPlacements,w=i.options.placement,x=W(w),O=l||(x===w||!y?[re(w)]:function(e){if("auto"===W(e))return[];var t=re(e);return[ie(e),t,ie(t)]}(w)),j=[w].concat(O).reduce((function(e,t){return e.concat("auto"===W(t)?ve(i,{placement:t,boundary:h,rootBoundary:m,padding:d,flipVariations:y,allowedAutoPlacements:b}):t)}),[]),E=i.rects.reference,D=i.rects.popper,A=new Map,L=!0,M=j[0],P=0;P<j.length;P++){var k=j[P],B=W(k),H="start"===K(k),R=[e,t].indexOf(B)>=0,T=R?"width":"height",S=me(i,{placement:k,boundary:h,rootBoundary:m,altBoundary:v,padding:d}),V=R?H?n:r:H?t:e;E[T]>D[T]&&(V=re(V));var q=re(V),C=[];if(c&&C.push(S[B]<=0),u&&C.push(S[V]<=0,S[q]<=0),C.every((function(e){return e}))){M=k,L=!1;break}A.set(k,C)}if(L)for(var N=function(e){var t=j.find((function(t){var n=A.get(t);if(n)return n.slice(0,e).every((function(e){return e}))}));if(t)return M=t,"break"},I=y?3:1;I>0;I--){if("break"===N(I))break}i.placement!==M&&(i.modifiersData[s]._skip=!0,i.placement=M,i.reset=!0)}},requiresIfExists:["offset"],data:{_skip:!1}};function ye(e,t,n){return void 0===n&&(n={x:0,y:0}),{top:e.top-t.height-n.y,right:e.right-t.width+n.x,bottom:e.bottom-t.height+n.y,left:e.left-t.width-n.x}}function be(o){return[e,n,t,r].some((function(e){return o[e]>=0}))}var we={name:"hide",enabled:!0,phase:"main",requiresIfExists:["preventOverflow"],fn:function(e){var t=e.state,n=e.name,r=t.rects.reference,o=t.rects.popper,i=t.modifiersData.preventOverflow,a=me(t,{elementContext:"reference"}),s=me(t,{altBoundary:!0}),f=ye(a,r),c=ye(s,o,i),p=be(f),u=be(c);t.modifiersData[n]={referenceClippingOffsets:f,popperEscapeOffsets:c,isReferenceHidden:p,hasPopperEscaped:u},t.attributes.popper=Object.assign({},t.attributes.popper,{"data-popper-reference-hidden":p,"data-popper-escaped":u})}};var xe={name:"offset",enabled:!0,phase:"main",requires:["popperOffsets"],fn:function(t){var o=t.state,i=t.options,a=t.name,s=i.offset,f=void 0===s?[0,0]:s,c=d.reduce((function(t,i){return t[i]=function(t,o,i){var a=W(t),s=[r,e].indexOf(a)>=0?-1:1,f="function"==typeof i?i(Object.assign({},o,{placement:t})):i,c=f[0],p=f[1];return c=c||0,p=(p||0)*s,[r,n].indexOf(a)>=0?{x:p,y:c}:{x:c,y:p}}(i,o.rects,f),t}),{}),p=c[o.placement],u=p.x,l=p.y;null!=o.modifiersData.popperOffsets&&(o.modifiersData.popperOffsets.x+=u,o.modifiersData.popperOffsets.y+=l),o.modifiersData[a]=c}};var Oe={name:"popperOffsets",enabled:!0,phase:"read",fn:function(e){var t=e.state,n=e.name;t.modifiersData[n]=he({reference:t.rects.reference,element:t.rects.popper,strategy:"absolute",placement:t.placement})},data:{}};var je={name:"preventOverflow",enabled:!0,phase:"main",fn:function(o){var i=o.state,a=o.options,s=o.name,f=a.mainAxis,c=void 0===f||f,p=a.altAxis,u=void 0!==p&&p,l=a.boundary,d=a.rootBoundary,h=a.altBoundary,m=a.padding,v=a.tether,g=void 0===v||v,y=a.tetherOffset,b=void 0===y?0:y,w=me(i,{boundary:l,rootBoundary:d,padding:m,altBoundary:h}),x=W(i.placement),O=K(i.placement),j=!O,E=_(x),D="x"===E?"y":"x",A=i.modifiersData.popperOffsets,L=i.rects.reference,M=i.rects.popper,P="function"==typeof b?b(Object.assign({},i.rects,{placement:i.placement})):b,H="number"==typeof P?{mainAxis:P,altAxis:P}:Object.assign({mainAxis:0,altAxis:0},P),R=i.modifiersData.offset?i.modifiersData.offset[i.placement]:null,T={x:0,y:0};if(A){if(c){var S,q="y"===E?e:r,C="y"===E?t:n,N="y"===E?"height":"width",I=A[E],F=I+w[q],U=I-w[C],Y=g?-M[N]/2:0,G="start"===O?L[N]:M[N],J="start"===O?-M[N]:-L[N],Q=i.elements.arrow,Z=g&&Q?V(Q):{width:0,height:0},$=i.modifiersData["arrow#persistent"]?i.modifiersData["arrow#persistent"].padding:{top:0,right:0,bottom:0,left:0},ee=$[q],te=$[C],ne=X(0,L[N],Z[N]),re=j?L[N]/2-Y-ne-ee-H.mainAxis:G-ne-ee-H.mainAxis,oe=j?-L[N]/2+Y+ne+te+H.mainAxis:J+ne+te+H.mainAxis,ie=i.elements.arrow&&z(i.elements.arrow),ae=ie?"y"===E?ie.clientTop||0:ie.clientLeft||0:0,se=null!=(S=null==R?void 0:R[E])?S:0,fe=I+oe-se,ce=X(g?B(F,I+re-se-ae):F,I,g?k(U,fe):U);A[E]=ce,T[E]=ce-I}if(u){var pe,ue="x"===E?e:r,le="x"===E?t:n,de=A[D],he="y"===D?"height":"width",ve=de+w[ue],ge=de-w[le],ye=-1!==[e,r].indexOf(x),be=null!=(pe=null==R?void 0:R[D])?pe:0,we=ye?ve:de-L[he]-M[he]-be+H.altAxis,xe=ye?de+L[he]+M[he]-be-H.altAxis:ge,Oe=g&&ye?function(e,t,n){var r=X(e,t,n);return r>n?n:r}(we,de,xe):X(g?we:ve,de,g?xe:ge);A[D]=Oe,T[D]=Oe-de}i.modifiersData[s]=T}},requiresIfExists:["offset"]};function Ee(e,t,n){void 0===n&&(n=!1);var r,o,i=L(t),a=L(t)&&function(e){var t=e.getBoundingClientRect(),n=H(t.width)/e.offsetWidth||1,r=H(t.height)/e.offsetHeight||1;return 1!==n||1!==r}(t),s=I(t),f=S(e,a,n),c={scrollLeft:0,scrollTop:0},p={x:0,y:0};return(i||!i&&!n)&&(("body"!==E(t)||fe(s))&&(c=(r=t)!==D(r)&&L(r)?{scrollLeft:(o=r).scrollLeft,scrollTop:o.scrollTop}:ae(r)),L(t)?((p=S(t,!0)).x+=t.clientLeft,p.y+=t.clientTop):s&&(p.x=se(s))),{x:f.left+c.scrollLeft-p.x,y:f.top+c.scrollTop-p.y,width:f.width,height:f.height}}function De(e){var t=new Map,n=new Set,r=[];function o(e){n.add(e.name),[].concat(e.requires||[],e.requiresIfExists||[]).forEach((function(e){if(!n.has(e)){var r=t.get(e);r&&o(r)}})),r.push(e)}return e.forEach((function(e){t.set(e.name,e)})),e.forEach((function(e){n.has(e.name)||o(e)})),r}var Ae={placement:"bottom",modifiers:[],strategy:"absolute"};function Le(){for(var e=arguments.length,t=new Array(e),n=0;n<e;n++)t[n]=arguments[n];return!t.some((function(e){return!(e&&"function"==typeof e.getBoundingClientRect)}))}function Me(e){void 0===e&&(e={});var t=e,n=t.defaultModifiers,r=void 0===n?[]:n,o=t.defaultOptions,i=void 0===o?Ae:o;return function(e,t,n){void 0===n&&(n=i);var o,a,s={placement:"bottom",orderedModifiers:[],options:Object.assign({},Ae,i),modifiersData:{},elements:{reference:e,popper:t},attributes:{},styles:{}},f=[],c=!1,p={state:s,setOptions:function(n){var o="function"==typeof n?n(s.options):n;u(),s.options=Object.assign({},i,s.options,o),s.scrollParents={reference:A(e)?pe(e):e.contextElement?pe(e.contextElement):[],popper:pe(t)};var a,c,l=function(e){var t=De(e);return j.reduce((function(e,n){return e.concat(t.filter((function(e){return e.phase===n})))}),[])}((a=[].concat(r,s.options.modifiers),c=a.reduce((function(e,t){var n=e[t.name];return e[t.name]=n?Object.assign({},n,t,{options:Object.assign({},n.options,t.options),data:Object.assign({},n.data,t.data)}):t,e}),{}),Object.keys(c).map((function(e){return c[e]}))));return s.orderedModifiers=l.filter((function(e){return e.enabled})),s.orderedModifiers.forEach((function(e){var t=e.name,n=e.options,r=void 0===n?{}:n,o=e.effect;if("function"==typeof o){var i=o({state:s,name:t,instance:p,options:r}),a=function(){};f.push(i||a)}})),p.update()},forceUpdate:function(){if(!c){var e=s.elements,t=e.reference,n=e.popper;if(Le(t,n)){s.rects={reference:Ee(t,z(n),"fixed"===s.options.strategy),popper:V(n)},s.reset=!1,s.placement=s.options.placement,s.orderedModifiers.forEach((function(e){return s.modifiersData[e.name]=Object.assign({},e.data)}));for(var r=0;r<s.orderedModifiers.length;r++)if(!0!==s.reset){var o=s.orderedModifiers[r],i=o.fn,a=o.options,f=void 0===a?{}:a,u=o.name;"function"==typeof i&&(s=i({state:s,options:f,name:u,instance:p})||s)}else s.reset=!1,r=-1}}},update:(o=function(){return new Promise((function(e){p.forceUpdate(),e(s)}))},function(){return a||(a=new Promise((function(e){Promise.resolve().then((function(){a=void 0,e(o())}))}))),a}),destroy:function(){u(),c=!0}};if(!Le(e,t))return p;function u(){f.forEach((function(e){return e()})),f=[]}return p.setOptions(n).then((function(e){!c&&n.onFirstUpdate&&n.onFirstUpdate(e)})),p}}var Pe=Me(),We=Me({defaultModifiers:[te,Oe,$,P]}),ke=Me({defaultModifiers:[te,Oe,$,P,xe,ge,je,J,we]});export{b as afterMain,v as afterRead,O as afterWrite,P as applyStyles,J as arrow,o as auto,i as basePlacements,g as beforeMain,h as beforeRead,w as beforeWrite,t as bottom,f as clippingParents,$ as computeStyles,ke as createPopper,Pe as createPopperBase,We as createPopperLite,me as detectOverflow,s as end,te as eventListeners,ge as flip,we as hide,r as left,y as main,j as modifierPhases,xe as offset,d as placements,p as popper,Me as popperGenerator,Oe as popperOffsets,je as preventOverflow,m as read,u as reference,n as right,a as start,e as top,l as variationPlacements,c as viewport,x as write};