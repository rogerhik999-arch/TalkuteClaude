// KeyboardViewController.swift
// Talkute Keyboard Extension for iOS
//
// A custom keyboard extension with voice input capabilities.

import UIKit
import AVFoundation

/// Main keyboard view controller for the Talkute keyboard extension
class KeyboardViewController: UIInputViewController {

    // MARK: - Properties

    private var microphoneButton: UIButton!
    private var isRecording = false
    private var audioRecorder: AVAudioRecorder?

    // MARK: - Lifecycle

    override func viewDidLoad() {
        super.viewDidLoad()
        setupKeyboardUI()
    }

    override func viewWillLayoutSubviews() {
        super.viewWillLayoutSubviews()
        setupKeyboardConstraints()
    }

    // MARK: - UI Setup

    private func setupKeyboardUI() {
        // Create keyboard container
        let keyboardView = UIView()
        keyboardView.backgroundColor = .systemGray6
        keyboardView.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(keyboardView)

        // Create microphone button
        microphoneButton = UIButton(type: .system)
        microphoneButton.setImage(UIImage(systemName: "mic.circle.fill"), for: .normal)
        microphoneButton.tintColor = .systemBlue
        microphoneButton.translatesAutoresizingMaskIntoConstraints = false
        microphoneButton.addTarget(self, action: #selector(microphoneButtonTapped), for: .touchDown)
        microphoneButton.addTarget(self, action: #selector(microphoneButtonReleased), for: [.touchUpInside, .touchUpOutside])
        keyboardView.addSubview(microphoneButton)

        // Create next keyboard button
        let nextKeyboardButton = UIButton(type: .system)
        nextKeyboardButton.setTitle("ABC", for: .normal)
        nextKeyboardButton.translatesAutoresizingMaskIntoConstraints = false
        nextKeyboardButton.addTarget(self, action: #selector(nextKeyboardButtonTapped), for: .touchUpInside)
        keyboardView.addSubview(nextKeyboardButton)

        // Setup constraints
        NSLayoutConstraint.activate([
            keyboardView.topAnchor.constraint(equalTo: view.topAnchor),
            keyboardView.leadingAnchor.constraint(equalTo: view.leadingAnchor),
            keyboardView.trailingAnchor.constraint(equalTo: view.trailingAnchor),
            keyboardView.bottomAnchor.constraint(equalTo: view.bottomAnchor),

            microphoneButton.centerXAnchor.constraint(equalTo: keyboardView.centerXAnchor),
            microphoneButton.centerYAnchor.constraint(equalTo: keyboardView.centerYAnchor),
            microphoneButton.widthAnchor.constraint(equalToConstant: 80),
            microphoneButton.heightAnchor.constraint(equalToConstant: 80),

            nextKeyboardButton.trailingAnchor.constraint(equalTo: keyboardView.trailingAnchor, constant: -16),
            nextKeyboardButton.bottomAnchor.constraint(equalTo: keyboardView.bottomAnchor, constant: -16)
        ])
    }

    private func setupKeyboardConstraints() {
        // Update constraints if needed
    }

    // MARK: - Actions

    @objc private func microphoneButtonTapped() {
        startRecording()
    }

    @objc private func microphoneButtonReleased() {
        stopRecording()
    }

    @objc private func nextKeyboardButtonTapped() {
        // Switch to next keyboard
        advanceToNextInputMode()
    }

    // MARK: - Recording

    private func startRecording() {
        guard !isRecording else { return }

        // Request microphone permission
        AVAudioSession.sharedInstance().requestRecordPermission { [weak self] granted in
            DispatchQueue.main.async {
                if granted {
                    self?.beginRecording()
                }
            }
        }
    }

    private func beginRecording() {
        isRecording = true
        microphoneButton.tintColor = .systemRed
        microphoneButton.setImage(UIImage(systemName: "mic.circle.fill"), for: .normal)

        // TODO: Start actual audio recording and send to Rust FFI
        // For now, just update UI
    }

    private func stopRecording() {
        guard isRecording else { return }

        isRecording = false
        microphoneButton.tintColor = .systemBlue
        microphoneButton.setImage(UIImage(systemName: "mic.circle.fill"), for: .normal)

        // TODO: Stop recording and process audio through Rust FFI
        // Then insert text at cursor

        // For demo, insert placeholder text
        insertText("Voice input text here")
    }

    // MARK: - Text Insertion

    private func insertText(_ text: String) {
        textDocumentProxy.insertText(text)
    }

    private func deleteText() {
        textDocumentProxy.deleteBackward()
    }
}