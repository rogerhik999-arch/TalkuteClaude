// TalkuteIME.kt
// Talkute Input Method Editor for Android
//
// A custom keyboard IME with voice input capabilities.

package com.talkute.ime

import android.inputmethodservice.InputMethodService
import android.view.View
import android.view.inputmethod.EditorInfo
import android.widget.Button
import android.widget.ImageButton
import android.Manifest
import android.content.pm.PackageManager
import android.media.AudioRecord
import android.media.MediaRecorder
import androidx.core.app.ActivityCompat

/**
 * Talkute Input Method Service
 *
 * Provides a custom keyboard with voice input functionality.
 */
class TalkuteIME : InputMethodService() {

    private var isRecording = false
    private var audioRecord: AudioRecord? = null

    override fun onCreateInputView(): View {
        // Inflate the keyboard layout
        val keyboardView = layoutInflater.inflate(R.layout.keyboard_layout, null)

        // Setup microphone button
        val micButton = keyboardView.findViewById<ImageButton>(R.id.mic_button)
        micButton?.setOnTouchListener { _, event ->
            when (event.action) {
                android.view.MotionEvent.ACTION_DOWN -> startRecording()
                android.view.MotionEvent.ACTION_UP, android.view.MotionEvent.ACTION_CANCEL -> stopRecording()
            }
            true
        }

        // Setup next keyboard button
        val nextKeyboardButton = keyboardView.findViewById<Button>(R.id.next_keyboard_button)
        nextKeyboardButton?.setOnClickListener {
            switchToNextInputMethod(false)
        }

        return keyboardView
    }

    override fun onStartInput(attribute: EditorInfo?, restarting: Boolean) {
        super.onStartInput(attribute, restarting)
        // Initialize input state
    }

    override fun onFinishInput() {
        super.onFinishInput()
        // Clean up input state
    }

    /**
     * Start voice recording
     */
    private fun startRecording() {
        if (isRecording) return

        // Check microphone permission
        if (ActivityCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO)
            != PackageManager.PERMISSION_GRANTED) {
            // Request permission through the main app
            return
        }

        isRecording = true

        // TODO: Initialize audio recording and send to Rust FFI
        // For now, just update UI state
    }

    /**
     * Stop voice recording and process audio
     */
    private fun stopRecording() {
        if (!isRecording) return

        isRecording = false

        // TODO: Stop recording and process audio through Rust FFI
        // Then insert text at cursor

        // For demo, insert placeholder text
        insertText("Voice input text here")
    }

    /**
     * Insert text at the current cursor position
     */
    private fun insertText(text: String) {
        currentInputConnection?.commitText(text, 1)
    }

    /**
     * Delete text before the cursor
     */
    private fun deleteText(length: Int = 1) {
        currentInputConnection?.deleteSurroundingText(length, 0)
    }

    /**
     * Get the text before the cursor
     */
    private fun getTextBeforeCursor(n: Int): String {
        return currentInputConnection?.getTextBeforeCursor(n, 0)?.toString() ?: ""
    }

    /**
     * Get the text after the cursor
     */
    private fun getTextAfterCursor(n: Int): String {
        return currentInputConnection?.getTextAfterCursor(n, 0)?.toString() ?: ""
    }
}