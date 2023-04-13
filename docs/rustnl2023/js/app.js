(function(){
  let slideshow = remark.create({
    sourceUrl: 'presentation.md'
  });

  slideshow.on('showSlide', function(slide){
    if (MathJax && MathJax.typeset){
      MathJax.typeset();
    }
  })
})();
