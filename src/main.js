const { invoke } = window.__TAURI__.tauri;
import WaveSurfer from 'https://unpkg.com/wavesurfer.js@7/dist/wavesurfer.esm.js'
import ZoomPlugin from "https://unpkg.com/wavesurfer.js@7/dist/plugins/zoom.esm.js"

//import { open } from '@tauri-apps/api/';
const { open } = window.__TAURI__.dialog;
const { convertFileSrc } = window.__TAURI__.dialog;

const audio = WaveSurfer.create({
  container: '.audio',
  waveColor: '#4F4A85',
  progressColor: '#383351',
  height:230,
  minPxPerSec: 0,
  //mediaControls: true,
})


// Initialize the Zoom plugin
audio.registerPlugin(
  ZoomPlugin.create({
    // the amount of zoom per wheel step, e.g. 0.5 means a 50% magnification per scroll
    scale: 10.0,
    // Optionally, specify the maximum pixels-per-second factor while zooming
    maxZoom: 100000,
  }),
)


////////////////////////////////// HTML ELEMENT //////////////////////////////////

const playBtn = document.querySelector(".play-btn");
const stopBtn = document.querySelector(".stop-btn");
const muteBtn = document.querySelector(".mute-btn");
const volumeSlider = document.querySelector(".volume-slider");

const menuTools = document.querySelector(".menu-tools");
const menuFilters = document.querySelector(".menu-filters");
const menuRecord = document.querySelector(".menu-record");
const menuFrequences = document.querySelector(".menu-frequences");
const menuCompression = document.querySelector(".menu-compression");
const menuNoiseReduction = document.querySelector(".menu-noise-reduction");
const menuInfos = document.querySelector(".menu-infos");
const menuGenerator = document.querySelector(".menu-generator");
const menuEqualiser = document.querySelector(".menu-egaliseur");
const menuAutotune = document.querySelector(".menu-autotune");

const menuLoad = document.querySelector(".submit")
const applyBtn = document.querySelector(".apply-btn");
const ToolBox = document.querySelector(".tools");

const pFilename = document.getElementById("pFilename");
const pChannels = document.getElementById("pChannels");
const pSampleRate = document.getElementById("pSampleRate");
const pBits = document.getElementById("pBits");
const pNSamples = document.getElementById("pNSamples");
const pDuration = document.getElementById("pDuration");
const pCompress = document.getElementById("compress-para");
const stylechoice = document.getElementById('choix');
const ambiancechoice = document.getElementById("ambiance-choix");

let greetInputEl;
let greetMsgEl;

let wav;
let fft_result;
let audio_path;
let name_file;
let chart_freq;
let done;
let DIR_PATH;
let ORIGINAL_DATA;
let ORIGINAL_HEADER;
let PREV_DATA = [];
let NEXT_DATA = [];
let wav_fusion;

////////////////////////////////// RUST BACKEND FUNCTIONS //////////////////////////////////

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function load(path) {
  done = false;
  audio_path = path;
  await invoke("load", {path:path}).then((data) => wav = data);
  //audio.load("sounds/"+path);
  
  ORIGINAL_DATA = wav[0];
  ORIGINAL_HEADER = wav[5];
  PREV_DATA.push([0,wav[0]]);
  pFilename.innerHTML = "Filename: "+name_file;
  pBits.innerHTML = "Bits per sample: "+wav[3];
  if (wav[1] == 1) {
    pChannels.innerHTML = "Channels: "+wav[1]+" (mono)";
  }
  else {
    pChannels.innerHTML = "Channels: "+wav[1]+ " (stereo)";
  }
  
  pDuration.innerHTML = "Duration: "+wav[4]+"s";
  pNSamples.innerHTML = "Number of samples: "+wav[0].length;
  pSampleRate.innerHTML = "Sample rate: "+wav[2]+"Hz";
  pCompress.innerHTML = "Size before compression : " + Math.round((wav[6]/1048576)*100)/100+"Mo";
  DIR_PATH = wav[7];
}

async function speed(n,callback) {
  PREV_DATA.push([5,wav[5]]);
  await invoke("speedUp", {header:wav[5],samplesRate:wav[2], n:parseFloat(n)}).then((newHeader) => wav[5] = newHeader);
  callback();
}

async function gain(db,callback) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("gainUp", {samples:wav[0],db:parseFloat(db)}).then((newSamples) => wav[0] = newSamples);
  callback();
}

async function save() {
  await invoke("save", {name:"result.wav", samples:wav[0], header:wav[5]});

}

async function apply_change() {
  await invoke("apply_change", {name:DIR_PATH+"/tmp.wav", samples:wav[0], header:wav[5]});
  let apiPath = window.__TAURI__.tauri.convertFileSrc(DIR_PATH+"/tmp.wav");
  console.log('API Path', apiPath);
  audio.load(apiPath);
}

async function generate(name,duration,freq,callback) {
  await invoke("generate_tone", {name:String(name), duration:duration, freq:parseFloat(freq)});
  callback();
}

async function fft() {
  done = true;
  await invoke("FFT", {samples:wav[0], samplesRate:wav[2], channels:wav[1]}).then((fft_res) => fft_result = fft_res);
}

async function NoiseReduction() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("noise_reduction", {samples:wav[0], m:31}).then((compress_data) => wav[0] = compress_data);
  apply_change();
}

function record() {
  invoke("record");
}

async function compress() {
  PREV_DATA.push([0,wav[0]]);
  let compress_data;
  await invoke("compress", {samples:wav[0]}).then((compress_data2) => compress_data = compress_data2);
  wav[0] = compress_data[0];
  pCompress.innerHTML += "</br> Size after compression :  :"+ (Math.round((compress_data[1]+44)/1048576*100))/100+"Mo";
  speed(0.5);
  apply_change();
}

async function reverse() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("reverse_sound", {samples:wav[0]}).then((reversed_samples) => wav[0] = reversed_samples);
  apply_change();
}

async function low_pass1(freq,callback) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("filtre_pass_bas", {samples:wav[0], freq:parseFloat(freq),samplesRate:wav[2]}).then((low_pass_samples) => wav[0] = low_pass_samples);
  callback();
}

async function high_pass1(freq,callback) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("filtre_pass_haut", {samples:wav[0], freq:parseFloat(freq),samplesRate:wav[2]}).then((high_pass_samples) => wav[0] = high_pass_samples);
  callback();
}

async function generate_melody(duration,callback) {
  await invoke("generate_melody", {duration:parseInt(duration)});
  callback();
}

async function generate_melody_midi(style,metric) {
  if (metric == "") {
    await invoke("generate_melody_midi", {style:String(style),metric:parseInt(0)});
  }
  else {
    await invoke("generate_melody_midi", {style:String(style),metric:parseInt(metric)});
  }
  
}

async function cut_left() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("right_only", {samples:wav[0]}).then((cut_channel) => wav[0] = cut_channel);
  apply_change();
}

async function cut_right() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("left_only", {samples:wav[0]}).then((cut_channel) => wav[0] = cut_channel);
  apply_change();
}

async function alternate() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("alternate", {samples:wav[0]}).then((cut_channel) => wav[0] = cut_channel);
  apply_change();
}

async function egaliseur(eps,min,max,db,callback) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("egalise", {s:wav[0],eps:parseInt(eps),db:parseFloat(db),lFreqMin:parseInt(min),lFreqMax:parseInt(max)}).then((new_samples) => wav[0] = new_samples);
  callback();
}

async function prev() {
  let data = PREV_DATA.pop();
  NEXT_DATA.push([wav[0],wav[5]]);
  wav[data[0]] = data[1];
  apply_change();
}

async function reset() {
  wav[0] = ORIGINAL_DATA;
  wav[5] = ORIGINAL_HEADER;
  apply_change();
}

async function forward() {
  let data = NEXT_DATA.pop();
  wav[0] = data[0];
  wav[5] = data[1];
  apply_change();
}

async function Echo() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("echo", {samples:wav[0]}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Fade() {
  PREV_DATA.push([0,wav[0]]);
  await invoke("fade", {samples:wav[0]}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Distortion(gain) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("distortion", {samples:wav[0],disto:parseFloat(gain)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Mute(start,end) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("mute", {samples:wav[0],start:parseFloat(start),end:parseFloat(end)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Ambiance(style) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("ambiance", {samples:wav[0],num:parseInt(style)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

// async function selectFiletoFusion() {
//   await window.__TAURI__.dialog.open().then( (localPath) => {
//     let apiPath = window.__TAURI__.tauri.convertFileSrc(localPath)
//     invoke("load", {path:localPath}).then((data) => wav_fusion = data);
//   })
// }

async function selectFiletoFusion() {
  return new Promise((resolve, reject) => {
      window.__TAURI__.dialog.open().then( (localPath) => {
        let apiPath = window.__TAURI__.tauri.convertFileSrc(localPath)
        invoke("load", {path:localPath}).then((data) => wav_fusion = data);
      })
      setTimeout(() => {
          resolve(wav_fusion);
      }, 8000); // Remplacez cela par le travail réel de sélection du fichier
  });
}

async function Fusion() {
  PREV_DATA.push([0,wav[0]]);
  await selectFiletoFusion();
  console.log(wav_fusion);
  await invoke("fusion", {samples1:wav[0],samples2:wav_fusion[0]}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Vibration(freq) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("vibration",{samples:wav[0],freq:parseFloat(freq)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Repeat1(start,end,n) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("repetition1",{samples:wav[0],start:parseFloat(start),end:parseFloat(end),n:parseInt(n)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Repeat2(start,end,n) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("repetition2",{samples:wav[0],start:parseFloat(start),end:parseFloat(end),n:parseInt(n)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

async function Shazam() {
  let name_song;
  await invoke("recognize_music",{samples:wav[0]}).then((name) => name_song = name);
  alert("This song seems to be from "+name_song);
}

async function Autotune(freq1,freq2) {
  PREV_DATA.push([0,wav[0]]);
  await invoke("autotune",{samples:wav[0],freq:parseInt(freq1),note:parseInt(freq2)}).then((new_samples) => wav[0] = new_samples);
  apply_change();
}

/*
window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
    
  });
});*/

////////////////////////////////// MEDIA CONTROLS //////////////////////////////////

playBtn.addEventListener("click", () => {
  audio.playPause();

  if (audio.isPlaying()) {
    playBtn.classList.add("playing");
  }
  else {
    playBtn.classList.remove("playing");
  }
});

stopBtn.addEventListener("click", () => {
  audio.stop();
  playBtn.classList.remove("playing");
});

volumeSlider.addEventListener("mouseup", () => {
  changeVolume(volumeSlider.value);
});

const changeVolume = (volume) => {
  if (volume == 0) {
    muteBtn.classList.add("muted");
  }
  else {
    muteBtn.classList.remove("muted");
  }
  audio.setVolume(volume)
};

muteBtn.addEventListener("click", () => {
  if (muteBtn.classList.contains("muted")) {
    muteBtn.classList.remove("muted");
    audio.setVolume(0.5);
    volumeSlider.value = 0.5;
  }
  else {
    audio.setVolume(0);
    muteBtn.classList.add("muted");
    volumeSlider.value = 0;
  }
  
});

////////////////////////////////// NAVIGATION BAR CLICK  //////////////////////////////////

/*document.getElementById("uploadBtn").addEventListener('change', function(e){
  var file = this.files[0].name;
  load(file);
});*/

async function getFile() {
  const fileInput = document.getElementById('uploadBtn');

  // Open a selection dialog for image files
  const selected = await open({
    multiple: true,
    filters: [{
      name: 'Image',
      extensions: ['png', 'jpeg', 'wav']
    }]
  });

  if (Array.isArray(selected)) {
    // user selected multiple files
  } else if (selected === null) {
    console.log("NONE");
  } else {
    console.log(selected);
  }
  console.log(selected);
  load(selected[0]);
  /*
  let apiPath = window.__TAURI__.tauri.convertFileSrc(selected);
  console.log('API Path', apiPath)*/
  //audio.load(apiPath);


  // Vérifier si un fichier a été sélectionné
  /*if (fileInput.files.length > 0) {
      const filePath = fileInput.files[0].name; // Obtient le nom du fichier
      console.log('Chemin d\'accès du fichier :', fileInput.files);
      load(filePath);
  } else {
      console.log('Aucun fichier sélectionné.');
  }*/
}

function HideAll() {
  let tools = document.querySelector(".tools");
  let filters = document.querySelector(".filters");
  let record = document.querySelector(".record");
  let freq = document.querySelector(".frequences");
  let compress = document.querySelector(".compression");
  let noise = document.querySelector(".noise-reduction");
  let infos = document.querySelector(".informations");
  let generators = document.querySelector(".generators");
  let equal = document.querySelector(".equalizer");
  let autotune = document.querySelector(".autotune");

  autotune.style.display = "none";
  tools.style.display = "none";
  filters.style.display = "none";
  record.style.display = "none";
  freq.style.display = "none";
  compress.style.display = "none";
  noise.style.display = "none";
  infos.style.display = "none";
  generators.style.display = "none";
  equal.style.display = "none";
}

menuTools.addEventListener("click", () => {
  let tools = document.querySelector(".tools");
  if (tools.style.display == "block") {
    tools.style.display = "none";
  }
  else {
    HideAll();
    tools.style.display = "block";
  }
});

menuFilters.addEventListener("click", () => {
  let filters = document.querySelector(".filters");
  if (filters.style.display == "block") {
    filters.style.display = "none";
  }
  else {
    HideAll();
    filters.style.display = "block";
  }
});

menuRecord.addEventListener("click", () => {
  let record = document.querySelector(".record");
  if (record.style.display == "block") {
    record.style.display = "none";
  }
  else {
    HideAll();
    record.style.display = "block";
  }
});

menuFrequences.addEventListener("click", () => {
  let freq = document.querySelector(".frequences");
  if (freq.style.display == "block") {
    freq.style.display = "none";
  }
  else {
    HideAll();
    freq.style.display = "block";
  }
});

menuCompression.addEventListener("click", () => {
  let compress = document.querySelector(".compression");
  if (compress.style.display == "block") {
    compress.style.display = "none";
  }
  else {
    HideAll();
    compress.style.display = "block";
  }
});

menuNoiseReduction.addEventListener("click", () => {
  let noise = document.querySelector(".noise-reduction");
  if (noise.style.display == "block") {
    noise.style.display = "none";
  }
  else {
    HideAll();
    noise.style.display = "block";
  }
});

menuInfos.addEventListener("click", () => {
  let infos = document.querySelector(".informations");
  if (infos.style.display == "block") {
    infos.style.display = "none";
  }
  else {
    HideAll();
    infos.style.display = "block";
  }
});

menuGenerator.addEventListener("click", () => {
  let infos = document.querySelector(".generators");
  if (infos.style.display == "block") {
    infos.style.display = "none";
  }
  else {
    HideAll();
    infos.style.display = "block";
  }
});

menuEqualiser.addEventListener("click", () => {
  let infos = document.querySelector(".equalizer");
  if (infos.style.display == "block") {
    infos.style.display = "none";
  }
  else {
    HideAll();
    infos.style.display = "block";
  }
});

menuAutotune.addEventListener("click", () => {
  let infos = document.querySelector(".autotune");
  if (infos.style.display == "block") {
    infos.style.display = "none";
  }
  else {
    HideAll();
    infos.style.display = "block";
  }
});

////////////////////////////////// APPLY BUTTONS  //////////////////////////////////

function GetInputTools() {
  // Récupérer l'élément input
  var input1 = document.getElementById("speed-input");
  var input2 = document.getElementById("gain-input");
  var input3 = document.getElementById("tone-input");
  var input4 = document.getElementById("melody-input");

  var MuteInputStart = document.getElementById("mute-input");
  var MuteInputEnd = document.getElementById("end-input");
  var DistortionInput = document.getElementById("distortion-input");

  var Repeat1StartInput = document.getElementById("repetition1-start-input");
  var Repeat1EndInput = document.getElementById("repetition1-end-input");
  var Repeat1CountInput = document.getElementById("repetition1-count-input");

  var VibrationInput = document.getElementById("vibration-input");
  
  // Récupérer la valeur de l'input
  var valeur_speed = input1.value;
  var valeur_gain = input2.value;
  var valeur_tone = input3.value;
  var valeur_melody = input4.value;

  if (valeur_speed.length > 0) {
    speed(valeur_speed,apply_change);
  }
  if (valeur_gain.length > 0) {
    gain(valeur_gain,apply_change);
  }
  if (valeur_tone.length > 0) {
    generate(valeur_tone,1,valeur_tone,apply_change);
  }
  if (stylechoice.value != "0") {
    //generate_melody(valeur_melody,apply_change);
    generate_melody_midi(stylechoice.value,valeur_melody)
  }

  if (MuteInputStart.value > 0) {
    Mute(MuteInputStart.value,MuteInputEnd.value);
  }

  if (DistortionInput.value > 0) {
    Distortion(DistortionInput.value);
  }
  if (ambiancechoice.value != "0") {
    Ambiance(ambiancechoice.value);
  }

  if (Repeat1StartInput.value.length > 0) {
    var checkbox = document.getElementById("Repeat2");
    if (checkbox.checked) {
      Repeat2(Repeat1StartInput.value,Repeat1EndInput.value,Repeat1CountInput.value);
    }
    else {
      Repeat1(Repeat1StartInput.value,Repeat1EndInput.value,Repeat1CountInput.value);
    }

  }

  if (VibrationInput.value.length > 0) {
    Vibration(VibrationInput.value);
  }
}

function GetFilters() {
  var low_pass_input = document.getElementById("low-input");
  var low_pass_value = low_pass_input.value;

  var high_pass_input = document.getElementById("high-input");
  var high_pass_value = high_pass_input.value;

  if (low_pass_value.length > 0) {
    low_pass1(low_pass_value,apply_change);
  }
  if (high_pass_value.length > 0) {
    high_pass1(high_pass_value,apply_change);
  }
}

function FFT2() {
  console.log(done);
  if (!done) {
    fft();
  }
  make_graph();
}

function make_graph() {
  let amplitudes;
  let pfreq = document.getElementById("analytic-freq");
  //pfreq.innerHTML = "Frequence analysis: "+fft_result+"hz";

  if(chart_freq) {
    chart_freq.destroy();
  }

  amplitudes = fft_result;

  const canvas = document.getElementById('myChart');

  /*canvas.width = 600; // Largeur
  canvas.height = 400; // Hauteur*/

  // Création du tableau des abscisses
  const labels = Array.from(Array(amplitudes.length).keys());

  // Création du graphique
  const ctx = document.getElementById('myChart').getContext('2d');
  chart_freq = new Chart(ctx, {
    type: 'line',
    data: {
      labels: labels, // Étiquettes sur l'axe des x
      datasets: [{
        label: 'Amplitudes',
        data: amplitudes,
        borderColor: 'blue',
        borderWidth: 1,
        fill: false
      }]
    },
    options: {
      scales: {
        y: {
          beginAtZero: true // L'axe des y commence à zéro
        }
      }
    }
  });
  done = true;
}

function ApplyAutotune() {
  var autotuneInput1 = document.getElementById("autotune-freq1-input");
  var autotuneInput2 = document.getElementById("autotune-freq2-input");

  if (autotuneInput1.value.length > 0) {
    Autotune(autotuneInput1.value,autotuneInput2.value);
  }
}

////////////////////////////////// EVENTS FOR BUTTON  //////////////////////////////////

document.getElementById('uploadBtn').addEventListener('click', playSelectedFileInit);
document.getElementById("savebtn").addEventListener("click", save);
document.getElementById("apply-btn1").addEventListener("click", GetInputTools);
document.getElementById("apply-btn2").addEventListener("click",GetFilters);
document.getElementById("apply-btn3").addEventListener("click", FFT2);
document.getElementById("apply-btn6").addEventListener("click",NoiseReduction);
document.getElementById("apply-btn4").addEventListener("click",record);
document.getElementById("apply-btn5").addEventListener("click",compress);
document.getElementById("reverse-sound").addEventListener("click",reverse);
document.getElementById("alternate-channels").addEventListener("click",alternate);
document.getElementById("cut-left").addEventListener("click",cut_left);
document.getElementById("cut-right").addEventListener("click",cut_right);
document.getElementById("prev-btn").addEventListener("click",prev);
document.getElementById("reset-btn").addEventListener("click",reset);
document.getElementById("forward-btn").addEventListener("click",forward);
document.getElementById("apply-btn7").addEventListener("click",GetInputTools);
document.getElementById("echo").addEventListener("click",Echo);
document.getElementById("fade").addEventListener("click",Fade);
document.getElementById("apply-btn8").addEventListener("click",balance);
document.getElementById("fusion").addEventListener("click",Fusion);
document.getElementById("detect-sound-btn").addEventListener("click",Shazam);
document.getElementById("apply-btn9").addEventListener("click",ApplyAutotune);

//Get the path of the file
//Convert the path to API-path
//Allow to load view from JS file
function playSelectedFileInit(event) {

  window.__TAURI__.dialog.open().then( (localPath) => {
    console.log("Video selected", localPath)
    
    let apiPath = window.__TAURI__.tauri.convertFileSrc(localPath)
    console.log('API Path', apiPath)
    audio.load(apiPath);
    load(localPath);
    let arr = localPath.split('\\');
    name_file = arr[arr.length - 1];
  })


  // getFile();
  // var file = this.files[0];
  // name_file=file.name

  // let fileURL = URL.createObjectURL(file);

  // audio.load(fileURL);
  
} ;

document.addEventListener('DOMContentLoaded', function() {
  const rangeInput = document.getElementById('min-freq-input');
  const rangeValue = document.getElementById('rangeValue1');

  const rangeInput2 = document.getElementById('max-freq-input');
  const rangeValue2 = document.getElementById('rangeValue2');

  const rangeInput3 = document.getElementById('gain-egal-input');
  const rangeValue3 = document.getElementById('rangeValue3');

  // Update the displayed value when the page loads
  rangeValue.textContent = rangeInput.value;
  rangeValue2.textContent = rangeInput2.value;
  rangeValue3.textContent = rangeInput3.value;

  // Add an event listener to update the value when the input changes
  rangeInput.addEventListener('input', function() {
      rangeValue.textContent = rangeInput.value;
  });
  rangeInput2.addEventListener('input', function() {
    rangeValue2.textContent = rangeInput2.value;
});

rangeInput3.addEventListener('input', function() {
  rangeValue3.textContent = rangeInput3.value;
});

});

stylechoice.addEventListener('change', function() {
  const selectedValue = stylechoice.value;
});

ambiancechoice.addEventListener('change', function() {
  const selectedValue = ambiancechoice.value;
});

function balance() {

  var input5 = document.getElementById("min-freq-input");
  var input6 = document.getElementById("max-freq-input");
  var input7 = document.getElementById("gain-egal-input");
  //var input8 = document.getElementById("fiability-input");

  egaliseur(4,input5.value,input6.value,input7.value,apply_change);
}