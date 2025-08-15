let currentHlsInstance = null;
let currentAudioElement = null;

export function attachHlsToAudio(audioId, streamUrl, volume) {
    const audio = document.getElementById(audioId);
    if (!audio) {
        console.error('Audio element not found:', audioId);
        return;
    }

    if (audio.hlsInstance) {
        audio.hlsInstance.destroy();
        audio.hlsInstance = null;
    }

    if (window.Hls && window.Hls.isSupported()) {
        const hls = new window.Hls();
        hls.loadSource(streamUrl);
        hls.attachMedia(audio);
        audio.hlsInstance = hls;
        
        currentHlsInstance = hls;
        currentAudioElement = audio;
        
        const validVolume = typeof volume === 'number' && isFinite(volume) ? Math.max(0, Math.min(1, volume)) : 1.0;
        audio.volume = validVolume;
        
        hls.on(window.Hls.Events.MANIFEST_PARSED, function() {
            audio.play().catch(e => console.error('Failed to play:', e));
        });
        
        hls.on(window.Hls.Events.ERROR, function(event, data) {
            console.error('HLS error:', data);
        });
    } else if (audio.canPlayType('application/vnd.apple.mpegurl')) {
        audio.src = streamUrl;
        const validVolume = typeof volume === 'number' && isFinite(volume) ? Math.max(0, Math.min(1, volume)) : 1.0;
        audio.volume = validVolume;
        audio.load();
        audio.play().catch(e => console.error('Failed to play:', e));
        currentAudioElement = audio;
    } else {
        console.error('HLS not supported in this browser');
    }
}

export function pauseRadio() {
    if (currentAudioElement) {
        currentAudioElement.pause();
        return true;
    }
    return false;
}

export function stopRadio() {
    if (currentAudioElement) {
        currentAudioElement.pause();
        currentAudioElement.currentTime = 0;
    }
    
    if (currentHlsInstance) {
        currentHlsInstance.destroy();
        currentHlsInstance = null;
    }
    
    currentAudioElement = null;
    return true;
}

export function setAudioVolume(volume) {
    if (currentAudioElement) {
        try {
            const validVolume = typeof volume === 'number' && isFinite(volume) ? Math.max(0, Math.min(1, volume)) : 1.0;
            currentAudioElement.volume = validVolume;
            return true;
        } catch (error) {
            console.error('Error setting volume:', error);
            return false;
        }
    }
    return false;
}