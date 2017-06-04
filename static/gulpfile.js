const gulp = require('gulp');

const postcss = require('gulp-postcss');
const sourcemaps = require('gulp-sourcemaps');
const autoprefixer = require('autoprefixer');
const babel = require('gulp-babel');
const concat = require('gulp-concat');

const uglify = require('gulp-uglify');

const fs = require('fs-extra');

const css_source = './app/css/';
const js_source = './app/js/';
const destination = './output/';

function addPrefix(element) {
  return "./lib/" + element;
}

const lib = require('./lib/lib.js');
lib.css = lib.css.map(addPrefix);
lib.js = lib.js.map(addPrefix);

/// QA

// > gulp css_qa
gulp.task('css_qa', function () {
  const csslint = require('gulp-csslint');
  const cssbeautify = require('gulp-cssbeautify');
  const csscomb = require('gulp-csscomb');
  const gulpStylelint = require('gulp-stylelint');

  gulp.src(css_source + '*.css')
  .pipe(csscomb())
  .pipe(cssbeautify({indent: '  '}))
  .pipe(postcss([autoprefixer()]))
  .pipe(gulp.dest(css_source));

  gulp.src(css_source + '*.css')
  .pipe(csslint())
  .pipe(csslint.reporter())
  .pipe(gulpStylelint({
    reporters: [
      {formatter: 'string', console: true},
    ]
  }));
});

// > gulp js_qa
gulp.task('js_qa', function () {
  const eslint = require('gulp-eslint');
  //const gulpIf = require('gulp-if');
  gulp.src(js_source + '*.js')
  .pipe(eslint({ fix: true }))
  .pipe(eslint.format())
  .pipe(eslint.failAfterError())
  //.pipe(gulpIf(isFixed, uglify())
  .pipe(gulp.dest(js_source));
});

// > gulp qa
gulp.task('qa', ['css_qa', 'js_qa']);

// Build
// > gulp css
gulp.task('css', function () {
  const cssnano = require('cssnano');
  var processors = [
    require("postcss-import")(),
    require("postcss-url")(),
    require("postcss-cssnext")({ warnForDuplicates: false }),
    require("postcss-browser-reporter")(),
    require("postcss-reporter")(),
    cssnano(),
  ];
  gulp.src(lib.css.concat(css_source + '*.css'))
  .pipe(postcss(processors))
  .pipe(sourcemaps.init())
  .pipe(postcss([autoprefixer()]))
  .pipe(concat('style.css'))
  .pipe(sourcemaps.write('.'))
  .pipe(gulp.dest(destination));
});

// > gulp lib_js
gulp.task('lib_js', function () {
  gulp.src(lib.js)
  .pipe(sourcemaps.init())
  .pipe(concat('lib.js'))
  .pipe(uglify())
  .pipe(sourcemaps.write('.'))
  .pipe(gulp.dest(destination));
});

// > gulp js
gulp.task('js', function () {
  gulp.src(js_source + '*.js')
  .pipe(sourcemaps.init())
  //.pipe(babel({ presets: ['es2015'] }))
  .pipe(concat('script.js'))
  .pipe(uglify())
  .pipe(sourcemaps.write('.'))
  .pipe(gulp.dest(destination));
});

// > gulp vue
gulp.task('vue', function () {
  gulp.src('./node_modules/vue/dist/vue.min.js')
  .pipe(sourcemaps.init())
  .pipe(sourcemaps.write('.'))
  .pipe(gulp.dest(destination));
});

// Watch Files For Changes :
// gulp watch
gulp.task('watch', function () {
  gulp.watch(css_source, ['css']);
  gulp.watch(js_source, ['lib_js']);
  gulp.watch(js_source, ['js']);
  gulp.watch(js_source, ['vue']);
});

// > gulp clean
gulp.task('clean', function () {
  fs.emptyDir(css_source, function (err) {
    if (!err) console.log('"' + css_source + '"' + ' cleaned!')
  });
  fs.emptyDir(js_source, function (err) {
    if (!err) console.log('"' + js_source + '"' + ' cleaned!')
  });
  fs.emptyDir(destination, function (err) {
    if (!err) console.log('"' + destination + '"' + ' cleaned!')
  });
});

// Default
// > gulp
gulp.task('default', ['css', 'lib_js', 'js', 'vue']);
