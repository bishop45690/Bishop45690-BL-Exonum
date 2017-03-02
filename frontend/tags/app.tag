<app>
    <div class="container">
        <div class="row">
            <div class="col-sm-6 col-sm-offset-3">
                <div class="panel panel-default">
                    <div class="panel-heading">
                        <div class="panel-title">
                            <div id="title" class="h4">{ title }</div>
                        </div>
                    </div>
                    <div id="content" class="panel-body"></div>
                </div>
            </div>
        </div>
    </div>

    <script>
        var self = this;
        var baseUrl = 'http://exonum.com/backends/currency/api/v1';
        var validators = [
            '7e2b6889b2e8b60e0e8d71be55b9cbf6aaa9bf397ef7b1d6b8564d862b120bea',
            '2f1e58c0752503e3b66a5f68d97ab44cac196c75608b53682c3da1f824f9391f',
            '8ce8ba0974e10d45d89b48a409015ebfe15a4aa9f9410951b266764b91c9d535',
            '11110c9c4b06d7cc0df9311aae089771b04b696a8eaa105ba39a186bcceed0c2'
        ];

        // business logic
        var cryptocurrency = Cryptocurrency();

        // global mixin with common functions and constants
        riot.mixin({
            api: {
                baseUrl: baseUrl,
                validators: validators,
                getWallet: function(publicKey, callback) {
                    $.ajax({
                        method: 'GET',
                        url: baseUrl + '/wallets/info?pubkey=' + publicKey,
                        success: function(response, textStatus, jqXHR) {
                            callback(cryptocurrency.parseBlock(publicKey, validators, response));
                        },
                        error: function(jqXHR, textStatus, errorThrown) {
                            console.error(textStatus);
                        }
                    });
                }
            },

            localStorage: {
                getUser: function() {},
                getUsers: function() {
                    return JSON.parse(window.localStorage.getItem('users')) || [];
                },
                setUser: function() {}
            },

            truncate: function(str, limit) {
                return (limit < str.length) ? str.substr(0, limit - 1) + '...' : str;
            },

            init: function() {
                this.on('mount', function() {
                    // add title if it is predefined in component
                    if (this.title) {
                        self.title = this.title;
                        self.update();
                    }
                });
            }
        });

        // shared observable to dynamically update app title
        var titleObservable = riot.observable();
        titleObservable.on('change', function(value) {
            self.title = value;
            self.update();
        });

        // init app routes
        this.on('mount', function() {

            route('/', function() {
                riot.mount('#content', 'welcome');
            });

            route('/register', function() {
                riot.mount('#content', 'register');
            });

            route('/user/*', function(publicKey) {
                riot.mount('#content', 'wallet', {publicKey: publicKey, titleObservable: titleObservable});
            });

            route('/user/*/transfer', function(publicKey) {
                riot.mount('#content', 'transfer', {publicKey: publicKey});
            });

            route('/user/*/add-funds', function(publicKey) {
                riot.mount('#content', 'add-funds', {publicKey: publicKey});
            });

            route('/blockchain', function() {
                riot.mount('#content', 'blockchain');
            });

            route('/blockchain/block/*', function(height) {
                riot.mount('#content', 'block', {height: height});
            });

            route('/blockchain/transaction/*', function(hash) {
                riot.mount('#content', 'transaction', {hash: hash, titleObservable: titleObservable});
            });

            route.start(true);
        });
    </script>
</app>