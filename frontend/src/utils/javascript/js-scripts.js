
//export function databaseInfo(name) {
//db.info().then(function (result) {
//}).catch(function (err) {
//  console.log(err);
//});
//}

export function createPouchDatabase(name) {
//var db = new PouchDB('http://localhost:5984/'+name, {
//  fetch: function (url, opts) {
//    opts.headers.set('X-Auth-CouchDB-UserName', 'foo');
//    opts.headers.set('X-Some-Special-Header', 'foo');
//    opts.headers.set('X-Some-Special-Header', 'foo');
//    return PouchDB.fetch(url, opts);
//  }
//});
//var db = new PouchDB('http://couched_visits:uX2b6@q5CxOjT7NrxYDc@localhost:5984/'+name);
//db.info()
var PouchDB = require('pouchdb-browser');
var db = new PouchDB('http://localhost:5984/' + name + '?name=couched_visits&password=uX2b6@q5CxOjT7NrxYDc', {skip_setup: true});
//var local = new PouchDB(name);
//local.sync(db, {live: true, retry: true}).on('error', console.log.bind(console));
}

export function replicateDatabase(name) {
var localDB = new PouchDB(name);
var remoteDB = new PouchDB('http://127.0.0.1:5984/' + name);
localDB.replicate.from(remoteDB,  { live: true, retry:true}).on('complete', function () {
  // yay, we're done!
}).on('error', function (err) {
  // boo, something went wrong!
});

}

export function getPayload() {
  return new Date().toString();
}

export function getPayloadLater(callback) {
  setTimeout(() => {
    callback(getPayload());
  }, 1000);
}

export function uikitNotify(msg, status) {
    UIkit.notification({
        message: msg,
        status: status,
        pos: 'bottom-right',
        timeout: 7000
    });
}

export function replaceLocationLogin() {
    window.location.replace("/tertiary#login");
}

export function toggle_uk_dropdown(element) {
    UIkit.dropdown(element).hide(false);
}

export function show_uk_modal(id) {
var modal = UIkit.modal(id);
  modal.show();
}

export function hide_uk_modal(id) {
var modal = UIkit.modal(id);
  modal.hide();
}

export function hide_uk_drop(element) {
    UIkit.drop(element).hide(false);
}

export function show_uk_drop(element) {
    UIkit.drop(element).show();
}

export function remove_class_name(element, class_name) {
       element.classList.remove(class_name);
}

export function add_class_name(element, class_name) {
       element.classList.add(class_name);
}

export function remove_element_by_id(id) {
    try {
         document.getElementById(id).remove();
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}

export function select_option(id, idx) {
    try {
         document.getElementById(id).options[idx].selected = true;
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}

export function copy_to_clipboard(id) {
    try {
         let elem = document.getElementById(id);
         elem.select();
         elem.setSelectionRange(0, 99999);
         elem.execCommand("copy");
         alert("Copied: " + elem.value);
    } catch (e) {
        console.log("Got exception ${e}" );
    }
}