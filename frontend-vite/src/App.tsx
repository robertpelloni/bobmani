import { useState } from 'react'

function App() {
  const [audioPath, setAudioPath] = useState('')
  const [status, setStatus] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  const handleGenerate = async () => {
    if (!audioPath) return
    setLoading(true)
    setStatus(null)

    try {
      const res = await fetch('/api/generate', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ audio_path: audioPath }),
      })
      const data = await res.json()
      setStatus(`[${data.status.toUpperCase()}]: ${data.message}`)
    } catch (e) {
      setStatus(`[ERROR]: Failed to connect to engine backend. Ensure Rust server is running.`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div style={{ padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h1 style={{ borderBottom: '1px solid #333', paddingBottom: '0.5rem' }}>Bobmani Engine</h1>
      <p style={{ color: '#ccc' }}>
        Universal Rhythm Engine configuration and AI generation suite.
        Input an audio file path to trace generating `.sm` targets via the `autochart` port.
      </p>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem', marginTop: '2rem' }}>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
          <label htmlFor="audioPath" style={{ fontWeight: 'bold' }}>Audio File Path</label>
          <input
            id="audioPath"
            type="text"
            placeholder="/path/to/song.mp3"
            value={audioPath}
            onChange={(e) => setAudioPath(e.target.value)}
            style={{
              padding: '0.75rem',
              borderRadius: '4px',
              border: '1px solid #444',
              backgroundColor: '#222',
              color: 'white'
            }}
          />
        </div>

        <button
          onClick={handleGenerate}
          disabled={loading || !audioPath}
          style={{
            padding: '0.75rem',
            backgroundColor: loading || !audioPath ? '#555' : '#4CAF50',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: loading || !audioPath ? 'not-allowed' : 'pointer',
            fontWeight: 'bold'
          }}
        >
          {loading ? 'Processing...' : 'Generate Chart'}
        </button>

        {status && (
          <div style={{
            marginTop: '1rem',
            padding: '1rem',
            backgroundColor: status.includes('ERROR') ? '#4a1111' : '#1b3b1b',
            border: status.includes('ERROR') ? '1px solid #ff4444' : '1px solid #4CAF50',
            borderRadius: '4px',
            fontFamily: 'monospace'
          }}>
            {status}
          </div>
        )}
      </div>
    </div>
  )
}

export default App
