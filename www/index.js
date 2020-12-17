import * as wasm from "gradient-descent-method";
import THREE,* as mb from "mathbox";

var mathbox = mathBox({
    plugins: ['core', 'cursor', 'mathbox', 'stats', 'controls'],
    controls: { klass: THREE.OrbitControls }
});
if (mathbox.fallback) throw "WebGL not supported"

var three = mathbox.three;
three.renderer.setClearColor(new THREE.Color(0x000000), 0.0);

// Event listener for when this is included in a revealjs presentation
window.addEventListener('message', function(e) {
  e = JSON.parse(e.data);
  if(e.eventName == 'slide') {
    present.set('index', e.value);
  }
});
// Place camera
var camera =
  mathbox
  .camera({
    proxy: true,
    position: [0, -3, 1],
  });

// Define global DOM handler to format 'latex' into an HTML span
MathBox.DOM.Types.latex = MathBox.DOM.createClass({
  render: function (el) {
    this.props.innerHTML = katex.renderToString(this.children);
    return el('span', this.props);
  }
});

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
async function removeCurrentSlideHtml(index) {
  selector = `.slide${index}-html`;
  console.log(selector);
  elements = document.querySelector(selector);
  if (elements != null) {
    document.querySelector(selector).style.display = "none";
    await sleep(1000);
    document.querySelector(selector).remove();
  }
}

// Define key right and left press events to move through slides
if (window == top)
  window.onkeydown = function (e) {
    switch (e.keyCode) {
      case 37:
      case 38:
        removeCurrentSlideHtml(present.get('index'));
        present.set('index', present.get('index') - 1);
        break;
      case 39:
      case 40:
        removeCurrentSlideHtml(present.get('index'));
        present.set('index', present.get('index') + 1);
        break;
    }
  }

// Define Button event functions
function previous() {
  removeCurrentSlideHtml(present.get('index'));
  present.set('index', present.get('index') - 1);
}

function next() {
  removeCurrentSlideHtml(present.get('index'));
  present.set('index', present.get('index') + 1);
}

var view = mathbox.cartesian({
    range: [[-3,3],[-3,3],[-3,3]],
    scale: [1,1,1]
})

view.axis({
    color: "pink",
}).scale({
    divide: 10
});
view.grid({
    axes: [1,2],
    width: 1,
    color: "lightblue",
})
view.grid({
    axes: [1,3],
    width: 1,
    color: "lightblue",
})

view.axis({
    color: "white",
    axis: 2
}).scale({
    divide: 10
});

view.axis({
    color: "white",
    axis: 3
}).scale({
    divide: 10
});

view.area({
    id: 'area-data',
    width: 100,
    height: 100,
    axes: [1,3],
    channels: 3,
    expr: function (emit, x, y, i, j) {
        emit(x,y,wasm.f(x,y));
    }
}).surface({
    // lineX: true,
    // lineY: true,
    shaded: true,
    color: 0x5090FF,
    width: 5,
})

// console.log(wasm.test());
var point = wasm.get_min(0.3, 0.3, 1);

view.interval({
  width: 1,
  channels: 3,
  expr: function (emit, x, i, t) {
    emit(point[0], point[1], wasm.f(point[0],point[1]));
  }
}).point({
  size: 20,
  color: "red",
  zBias: 2,
})

export function draw_a_line() {
  view.interval({
    width: 100,
    channels:2,
    expr: function (emit, x, i, t) {
      emit(x,y);
    }
  }).line({
    color: 0x30C0FF,
    width: 16
  });
}
