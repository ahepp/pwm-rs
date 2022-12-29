pipeline {
    agent {
        dockerfile {
            args '-v ${WORKSPACE}:/ws'
        }
    }
    stages {
        stage('Build') {
            steps {
                sh 'cargo clean'
                sh 'cargo build'
            }
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}
