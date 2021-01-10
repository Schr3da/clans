import React from 'react';

import {
  SafeAreaView,
  StyleSheet,
  View,
  Text,
  StatusBar,
} from 'react-native';

const styles = StyleSheet.create({
  container: {
    justifyContent: 'center',
    alignItems: 'center',
    display: 'flex',
  },
});

const App = () => {
  return (
    <>
      <SafeAreaView>
        <StatusBar barStyle="dark-content" />
        <View style={styles.container}>
          <Text>Upcoming Game</Text>
        </View>
      </SafeAreaView>
    </>
  );
};

export default App;
