package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/internal/openapi"
)

type (
	ListResponseEndpointOut   = openapi.ListResponseEndpointOut
	EndpointIn                = openapi.EndpointIn
	EndpointUpdate            = openapi.EndpointUpdate
	EndpointOut               = openapi.EndpointOut
	EndpointPatch             = openapi.EndpointPatch
	EndpointSecretOut         = openapi.EndpointSecretOut
	EndpointSecretRotateIn    = openapi.EndpointSecretRotateIn
	EndpointTransformationIn  = openapi.EndpointTransformationIn
	RecoverIn                 = openapi.RecoverIn
	ReplayIn                  = openapi.ReplayIn
	EndpointHeadersIn         = openapi.EndpointHeadersIn
	EndpointHeadersPatchIn    = openapi.EndpointHeadersPatchIn
	EndpointHeadersOut        = openapi.EndpointHeadersOut
	EndpointStats             = openapi.EndpointStats
	EndpointTransformationOut = openapi.EndpointTransformationOut
	EventExampleIn            = openapi.EventExampleIn
	Ordering                  = openapi.Ordering
	RecoverOut                = openapi.RecoverOut
	ReplayOut                 = openapi.ReplayOut
)

type Endpoint struct {
	api *openapi.APIClient
}

type EndpointListOptions struct {
	// Limit the number of returned items
	Limit *int32
	// The iterator returned from a prior invocation
	Iterator *string
	// The sorting order of the returned items
	Order *Ordering
}

type EndpointStatsOptions struct {
	// Filter the range to data starting from this date
	Since *time.Time
	// Filter the range to data ending by this date
	Until *time.Time
}

func (e *Endpoint) List(ctx context.Context, appId string, options *EndpointListOptions) (*ListResponseEndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointList(ctx, appId)
	if options != nil {
		if options.Iterator != nil {
			req = req.Iterator(*options.Iterator)
		}
		if options.Limit != nil {
			req = req.Limit(*options.Limit)
		}
		if options.Order != nil {
			req = req.Order(*options.Order)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) Create(ctx context.Context, appId string, endpointIn *EndpointIn) (*EndpointOut, error) {
	return e.CreateWithOptions(ctx, appId, endpointIn, nil)
}

func (e *Endpoint) CreateWithOptions(ctx context.Context, appId string, endpointIn *EndpointIn, options *PostOptions) (*EndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointCreate(ctx, appId)
	req = req.EndpointIn(*endpointIn)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) Get(ctx context.Context, appId string, endpointId string) (*EndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointGet(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) Update(ctx context.Context, appId string, endpointId string, endpointUpdate *EndpointUpdate) (*EndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointUpdate(ctx, appId, endpointId)
	req = req.EndpointUpdate(*endpointUpdate)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) Patch(ctx context.Context, appId string, endpointId string, endpointPatch *EndpointPatch) (*EndpointOut, error) {
	req := e.api.EndpointAPI.V1EndpointPatch(ctx, appId, endpointId)
	req = req.EndpointPatch(*endpointPatch)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) Delete(ctx context.Context, appId string, endpointId string) error {
	req := e.api.EndpointAPI.V1EndpointDelete(ctx, appId, endpointId)
	res, err := req.Execute()
	return wrapError(err, res)
}

func (e *Endpoint) GetSecret(ctx context.Context, appId string, endpointId string) (*EndpointSecretOut, error) {
	req := e.api.EndpointAPI.V1EndpointGetSecret(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) RotateSecret(ctx context.Context, appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn) error {
	return e.RotateSecretWithOptions(ctx, appId, endpointId, endpointSecretRotateIn, nil)
}

func (e *Endpoint) RotateSecretWithOptions(ctx context.Context, appId string, endpointId string, endpointSecretRotateIn *EndpointSecretRotateIn, options *PostOptions) error {
	req := e.api.EndpointAPI.V1EndpointRotateSecret(ctx, appId, endpointId)
	req = req.EndpointSecretRotateIn(*endpointSecretRotateIn)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) Recover(ctx context.Context, appId string, endpointId string, recoverIn *RecoverIn) (*RecoverOut, error) {
	return e.RecoverWithOptions(ctx, appId, endpointId, recoverIn, nil)
}

func (e *Endpoint) RecoverWithOptions(ctx context.Context, appId string, endpointId string, recoverIn *RecoverIn, options *PostOptions) (*RecoverOut, error) {
	req := e.api.EndpointAPI.V1EndpointRecover(ctx, appId, endpointId)
	req = req.RecoverIn(*recoverIn)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) GetHeaders(ctx context.Context, appId string, endpointId string) (*EndpointHeadersOut, error) {
	req := e.api.EndpointAPI.V1EndpointGetHeaders(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) UpdateHeaders(ctx context.Context, appId string, endpointId string, endpointHeadersIn *EndpointHeadersIn) error {
	req := e.api.EndpointAPI.V1EndpointUpdateHeaders(ctx, appId, endpointId)
	req = req.EndpointHeadersIn(*endpointHeadersIn)
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) PatchHeaders(ctx context.Context, appId string, endpointId string, endpointHeadersIn *EndpointHeadersPatchIn) error {
	req := e.api.EndpointAPI.V1EndpointPatchHeaders(ctx, appId, endpointId)
	req = req.EndpointHeadersPatchIn(*endpointHeadersIn)
	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}
	return nil
}

func (e *Endpoint) GetStats(ctx context.Context, appId string, endpointId string) (*EndpointStats, error) {
	return e.GetStatsWithOptions(ctx, appId, endpointId, EndpointStatsOptions{})
}

func (e *Endpoint) GetStatsWithOptions(ctx context.Context, appId string, endpointId string, options EndpointStatsOptions) (*EndpointStats, error) {
	req := e.api.EndpointAPI.V1EndpointGetStats(ctx, appId, endpointId)
	if options.Since != nil {
		req = req.Since(*options.Since)
	}
	if options.Until != nil {
		req = req.Until(*options.Until)
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) ReplayMissing(ctx context.Context, appId string, endpointId string, replayIn *ReplayIn) (*ReplayOut, error) {
	return e.ReplayMissingWithOptions(ctx, appId, endpointId, replayIn, nil)
}

func (e *Endpoint) ReplayMissingWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn *ReplayIn,
	options *PostOptions,
) (*ReplayOut, error) {
	req := e.api.EndpointAPI.V1EndpointReplayMissing(ctx, appId, endpointId)
	req.ReplayIn(*replayIn)
	if options != nil {
		if options.IdempotencyKey != nil {
			req = req.IdempotencyKey(*options.IdempotencyKey)
		}
	}
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) TransformationGet(ctx context.Context, appId string, endpointId string) (*EndpointTransformationOut, error) {
	req := e.api.EndpointAPI.V1EndpointTransformationGet(ctx, appId, endpointId)
	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}

func (e *Endpoint) TransformationPartialUpdate(ctx context.Context, appId string, endpointId string, transformation *EndpointTransformationIn) error {
	req := e.api.EndpointAPI.V1EndpointTransformationPartialUpdate(ctx, appId, endpointId)
	req = req.EndpointTransformationIn(*transformation)

	res, err := req.Execute()
	if err != nil {
		return wrapError(err, res)
	}

	return nil
}

func (e *Endpoint) SendExample(ctx context.Context, appId string, endpointId string, eventExampleIn *EventExampleIn) (*MessageOut, error) {
	return e.SendExampleWithOptions(ctx, appId, endpointId, eventExampleIn, nil)
}

func (e *Endpoint) SendExampleWithOptions(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn *EventExampleIn,
	options *PostOptions,
) (*MessageOut, error) {
	req := e.api.EndpointAPI.V1EndpointSendExample(ctx, appId, endpointId)
	req.EventExampleIn(*eventExampleIn)

	if options != nil {
		if options.IdempotencyKey != nil {
			req.IdempotencyKey(*options.IdempotencyKey)
		}
	}

	ret, res, err := req.Execute()
	if err != nil {
		return nil, wrapError(err, res)
	}
	return ret, nil
}
