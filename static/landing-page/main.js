'use strict';

document.addEventListener('DOMContentLoaded', function () {
  // Toggles

  var $burgers = getAll('.burger');

  if ($burgers.length > 0) {
    $burgers.forEach(function ($el) {
      $el.addEventListener('click', function () {
        var target = $el.dataset.target;
        var $target = document.getElementById(target);
        $el.classList.toggle('is-active');
        $target.classList.toggle('is-active');
      });
    });
  }

  // Get all document sliders
  var sliders = document.querySelectorAll( 'input[type="range"].slider' );
  
  [].forEach.call( sliders, function ( slider ) {
    var output = findOutputForSlider( slider );
    if ( output ) {
      if ( slider.classList.contains( 'has-output-tooltip' ) ) {
        // Get new output position
        var newPosition = getSliderOutputPosition( slider );

        // Set output position
        output.style[ 'left' ] = newPosition.position;
      }

      // Add event listener to update output when slider value change
      slider.addEventListener( 'input', function( event ) {
        if ( event.target.classList.contains( 'has-output-tooltip' ) ) {
          // Get new output position
          var newPosition = getSliderOutputPosition( event.target );

          // Set output position
          output.style[ 'left' ] = newPosition.position;
        }

        // Update output with slider value
        output.value = event.target.value + '€';
      });
    }
  });

  // Functions
  function getAll(selector) {
    return Array.prototype.slice.call(document.querySelectorAll(selector), 0);
  }
});


