const gulp = require('gulp');

const postcss = require('gulp-postcss');
const sourcemaps = require('gulp-sourcemaps');
const babel = require('gulp-babel');
const concat = require('gulp-concat');
const uglify = require('gulp-uglify');
const csslint = require('gulp-csslint');
const cssbeautify = require('gulp-cssbeautify');
const csscomb = require('gulp-csscomb');
const eslint = require('gulp-eslint');

const riot = require('gulp-riot');

const cssnano = require('cssnano');

const fs = require('fs-extra');

const css_source = './app/css/';
const js_source = './app/js/';
const riot_source = './app/tags/';
const destination = './output/';

function addPrefix(element) {
    return "./lib/" + element;
}

const lib = require('./lib/lib.js');
lib.css = lib.css.map(addPrefix);
lib.js = lib.js.map(addPrefix);

// QA
gulp.task('css_qa', function () {
    gulp.src(css_source + '*.css')
    .pipe(csscomb())
    .pipe(cssbeautify({indent: '  '}))
    .pipe(gulp.dest(css_source));

    gulp.src(css_source + '*.css')
    .pipe(csslint())
    .pipe(csslint.reporter());
});

gulp.task('js_qa', function () {
    gulp.src(js_source + '*.js')
    .pipe(eslint({ fix: true }))
    .pipe(eslint.format())
    .pipe(eslint.failAfterError())
    .pipe(gulpIf(isFixed, gulp.dest(js_source)));
});

gulp.task('qa', ['css_qa', 'js_qa']);

// Build
gulp.task('css', function () {
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
    .pipe(sourcemaps.write('.'))
    .pipe(gulp.dest(destination));
});

gulp.task('js', function () {
    gulp.src(lib.js.concat(js_source + '*.js'))
    .pipe(sourcemaps.init())
    .pipe(babel({ presets: ['es2015'] }))
    .pipe(concat('script.js'))
    .pipe(uglify())
    .pipe(sourcemaps.write('.'))
    .pipe(gulp.dest(destination));
});

gulp.task('riot', function () {
    gulp.src('./node_modules/riot/riot.min.js')
    .pipe(gulp.dest(destination));

    gulp.src(riot_source + '*.tag')
    .pipe(sourcemaps.init())
    .pipe(riot())
    .pipe(concat('tags.js'))
    .pipe(uglify())
    .pipe(sourcemaps.write('.'))
    .pipe(gulp.dest(destination));
});

// Watch Files For Changes
gulp.task('watch', function () {
    gulp.watch(css_source, ['css']);
    gulp.watch(js_source, ['js']);
    gulp.watch(js_source, ['riot']);
});

// Clean
gulp.task('clean', function () {
    fs.emptyDir(css_source, function (err) {
      if (!err) console.log('"' + css_source + '"' + ' cleaned!')
    });
    fs.emptyDir(js_source, function (err) {
      if (!err) console.log('"' + js_source + '"' + ' cleaned!')
    });
    fs.emptyDir(riot_source, function (err) {
      if (!err) console.log('"' + riot_source + '"' + ' cleaned!')
    });
    fs.emptyDir(destination, function (err) {
      if (!err) console.log('"' + destination + '"' + ' cleaned!')
    });
});

// Default
gulp.task('default', ['css', 'js', 'riot']);
