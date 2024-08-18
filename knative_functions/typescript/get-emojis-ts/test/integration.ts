'use strict';
import { start, InvokerOptions } from 'faas-js-runtime';
import request from 'supertest';

import * as func from '../build';
import test, { Test } from 'tape';

const errHandler = (t: Test) => (err: Error) => {
  t.error(err);
  t.end();
};

test('Integration: handles a valid request with query parameter', (t) => {
  const expected = '{"flame":"fire,🔥","hound":"dog,🐕"}';
  start(func.handle, {} as InvokerOptions).then((server) => {
    t.plan(3);
    request(server)
      .post('/')
      .query({ descriptions: 'flame,hound' }) // Add the query parameter here
      .expect(200)
      .expect('Content-Type', /json/)
      .end((err, result) => {
        t.error(err, 'No error');
        t.ok(result);
        t.equal(expected, JSON.stringify(result.body));
        t.end();
        server.close();
      });
  }, errHandler(t));
});
