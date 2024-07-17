import init, { convert_epoch_to_date, parse_cron_expression } from './pkg/kya_pata.js';

async function run() {
    await init();
}

window.convertEpoch = function() {
    const epoch = parseInt(document.getElementById('epochInput').value);
    document.getElementById('dateOutput').textContent = convert_epoch_to_date(epoch);
}

window.parseCron = function() {
    const cron = document.getElementById('cronInput').value;
    document.getElementById('cronOutput').textContent = parse_cron_expression(cron);
}

run();
